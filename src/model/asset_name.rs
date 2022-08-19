use std::env;

/// Part of the name for each OS to identify proper asset
#[derive(Debug)]
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
      "macos"   => self.macos.as_ref(),
      _         => self.linux.as_ref(),
    }
  }
}
