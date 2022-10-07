use std::env;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum OS {
    Windows,
    MacOS,
    Linux,
}

/// Return the current OS where the 'tool-sync' is running
///
/// !!! WARNING !!! This function uses OS of the system where 'tool-sync' was
/// compiled. The function relies on the assumption that a user will run e.g.
/// the macOS executable on macOS
pub fn get_current_os() -> OS {
    match env::consts::OS {
        "windows" => OS::Windows,
        "macos" => OS::MacOS,
        _ => OS::Linux,
    }
}

impl Display for OS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Windows => {
                write!(f, "windows")
            }
            Self::MacOS => {
                write!(f, "macos")
            }
            Self::Linux => {
                write!(f, "linux")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_os() {
        let os = get_current_os();

        if cfg!(target_os = "windows") {
            assert_eq!(os, OS::Windows);
        } else if cfg!(target_os = "macos") {
            assert_eq!(os, OS::MacOS);
        } else {
            assert_eq!(os, OS::Linux);
        }
    }

    #[test]
    fn os_display() {
        assert_eq!(OS::Windows.to_string(), String::from("windows"));
        assert_eq!(OS::MacOS.to_string(), String::from("macos"));
        assert_eq!(OS::Linux.to_string(), String::from("linux"));
    }
}
