use std::env;

/// Part of the name for each OS to identify proper asset
#[derive(Debug, PartialEq)]
pub struct AssetName {
    pub linux: Option<String>,
    pub macos: Option<String>,
    pub windows: Option<String>,
}

impl AssetName {
    /// Get the current OS where the 'tool-sync' is running and extract the
    /// corresponding name of the downloaded tool
    ///
    /// !!! WARNING !!! This function uses OS of the system where 'tool-sync' was
    /// compiled. The function relies on the assumption that a user will run e.g.
    /// the macOS executable on macOS
    pub fn get_name_by_os(&self) -> Option<&String> {
        match env::consts::OS {
            "windows" => self.windows.as_ref(),
            "macos" => self.macos.as_ref(),
            _ => self.linux.as_ref(),
        }
    }
}

/// Add .exe extension to executables on Windows
pub fn mk_exe_name(exe_name: &str) -> String {
    let windows_exe_name: String = format!("{exe_name}.exe");

    if cfg!(windows) {
        windows_exe_name
    } else {
        exe_name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exe_name() {
        let exe_name = mk_exe_name("my-name");

        if cfg!(windows) {
            assert_eq!(exe_name, "my-name.exe");
        } else {
            assert_eq!(exe_name, "my-name");
        }
    }

    #[test]
    fn asset_name() {
        let asset_name = AssetName {
            linux: Some(String::from("oh-my-zsh")),
            macos: Some(String::from("fish")),
            windows: Some(String::from("powershell")),
        };

        let name = asset_name.get_name_by_os();

        if cfg!(target_os = "windows") {
            assert_eq!(name, Some(&String::from("powershell")));
        } else if cfg!(target_os = "macos") {
            assert_eq!(name, Some(&String::from("fish")));
        } else {
            assert_eq!(name, Some(&String::from("oh-my-zsh")));
        }
    }
}
