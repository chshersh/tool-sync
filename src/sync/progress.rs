use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct SyncProgress {
    max_tool_size: usize,
    max_tag_size: usize,
    multi_progress: MultiProgress,
}

const SUCCESS: Emoji<'_, '_> = Emoji("✅  ", "OK ");
const FAILURE: Emoji<'_, '_> = Emoji("⛔  ", "NO ");
const PROCESS: Emoji<'_, '_> = Emoji("📥  ", ".. ");

const MIN_TAG_SIZE: usize = 8;

/// A data type with minimal information required to create `SyncProgress`
pub struct ToolPair<'a> {
    pub name: &'a str,
    pub tag: &'a str,
}

impl SyncProgress {
    /// Creates new `SyncProgress` from a list of tools.
    /// !!! The given `Vec` must be non-empty !!!
    pub fn new(tool_pairs: Vec<ToolPair>) -> SyncProgress {
        // unwrap is safe here because 'new' is called with a non-empty vector
        let max_tool_size = tool_pairs.iter().map(|tp| tp.name.len()).max().unwrap();

        // putting a default of 8 here since tags like v0.10.10 is already 8
        let max_tag_size = tool_pairs
            .iter()
            .map(|tp| std::cmp::max(tp.tag.len(), MIN_TAG_SIZE))
            .max()
            .unwrap_or(MIN_TAG_SIZE);

        let multi_progress = MultiProgress::new();

        SyncProgress {
            max_tool_size,
            max_tag_size,
            multi_progress,
        }
    }

    fn fmt_prefix(&self, emoji: Emoji, tool_name: &str, tag: &str) -> String {
        let aligned_tool = format!(
            "{:tool_width$} {:tag_width$}",
            tool_name,
            tag,
            tool_width = self.max_tool_size,
            tag_width = self.max_tag_size,
        );

        format!("{}{}", emoji, aligned_tool)
    }

    pub fn create_message_bar(&self, tool_name: &str, tag: &str) -> ProgressBar {
        let message_style = ProgressStyle::with_template("{prefix:.bold.dim} {msg}").unwrap();

        self.multi_progress.add(
            ProgressBar::new(100)
                .with_style(message_style)
                .with_prefix(self.fmt_prefix(PROCESS, tool_name, tag)),
        )
    }

    pub fn create_progress_bar(&self, size: u64) -> ProgressBar {
        let bar_style =
            ProgressStyle::with_template("{bytes}/{total_bytes} {wide_bar:.cyan/blue}").unwrap();

        self.multi_progress
            .add(ProgressBar::new(size).with_style(bar_style))
    }

    pub fn finish_progress(pb: ProgressBar) {
        pb.finish_and_clear()
    }

    pub fn success(&self, pb: ProgressBar, tool_name: &str, tag: &str) {
        pb.set_prefix(self.fmt_prefix(SUCCESS, tool_name, tag));

        let success_msg = format!("{}", style("Completed!").bold().green());
        pb.set_message(success_msg);
        pb.finish();
    }

    pub fn failure(&self, pb: ProgressBar, tool_name: &str, tag: &str, err_msg: String) {
        pb.set_prefix(self.fmt_prefix(FAILURE, tool_name, tag));

        let failure_msg = format!("{}", style(err_msg).red());
        pb.set_message(failure_msg);
        pb.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_tag_size_specific() {
        let tool_pairs = vec![
            ToolPair {
                name: "ripgrep",
                tag: "v10.10.100",
            },
            ToolPair {
                name: "bat",
                tag: "latest",
            },
            ToolPair {
                name: "exa",
                tag: "latest",
            },
        ];

        let progress = SyncProgress::new(tool_pairs);

        // v10.10.100 is 10 characters
        assert_eq!(progress.max_tag_size, 10);
        // ripgrep is 7 characters
        assert_eq!(progress.max_tool_size, 7);
    }

    #[test]
    fn test_max_tag_size_latest() {
        let tool_pairs = vec![
            ToolPair {
                name: "ripgrep",
                tag: "latest",
            },
            ToolPair {
                name: "bat",
                tag: "latest",
            },
            ToolPair {
                name: "exa",
                tag: "latest",
            },
        ];

        let progress = SyncProgress::new(tool_pairs);

        // latest is 6 characters so it should default to 8
        assert_eq!(progress.max_tag_size, MIN_TAG_SIZE);
        // ripgrep is 7 characters
        assert_eq!(progress.max_tool_size, 7);
    }
}
