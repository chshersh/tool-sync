/// The maximum allowed threshold edit distance
/// All the values above that will be dropped
///
/// WARNING: Updating the constant would probably break the tests below,
/// so please change them accordingly
pub const EDIT_DISTANCE_THRESHOLD: usize = 4;

/// Calculate the Levenshtein \ Edit distance between two strings
///
/// The implementation is a copy from
/// [edit-distance] https://crates.io/crates/edit-distance
pub fn edit_distance(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    if len_a < len_b {
        return edit_distance(b, a);
    }
    // handle special case of 0 length
    if len_a == 0 {
        return len_b;
    } else if len_b == 0 {
        return len_a;
    }

    let len_b = len_b + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    #[allow(clippy::all)]
    for i in 1..len_b {
        cur[i] = i;
    }

    // calculate edit distance
    for (i, ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1,
                std::cmp::min(
                    // insertion
                    cur[j] + 1,
                    // match or substitution
                    pre + usize::from(ca != cb),
                ),
            );
            pre = tmp;
        }
    }
    cur[len_b - 1]
}

/// Find and return closest string based on the lowest edit distance between source and every possible value
pub fn closest_string(source: String, possible_values: Vec<String>) -> Option<String> {
    possible_values
        .iter()
        .filter(|value| source.len().abs_diff(value.len()) <= EDIT_DISTANCE_THRESHOLD)
        .map(|value| (value, edit_distance(&source, value)))
        .filter(|item| item.1 <= EDIT_DISTANCE_THRESHOLD)
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_string_result() {
        let possible_string = closest_string(
            "v13.0.0".to_string(),
            vec![
                "13.0.1".to_string(),
                "13.0.0".to_string(),
                "13.0.2".to_string(),
                "12.0.0".to_string(),
            ],
        );

        assert_eq!(Some("13.0.0").as_deref(), possible_string.as_deref());
    }

    #[test]
    fn test_closest_string_none() {
        let possible_string = closest_string(
            "v13.0.10".to_string(),
            vec![
                "v13".to_string(),
                "v24.1.22".to_string(),
                "23.1.21".to_string(),
            ],
        );

        assert_eq!(None::<String>.as_deref(), possible_string.as_deref());
    }
}
