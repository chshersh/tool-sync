use std::collections::HashMap;
use std::path::PathBuf;
use std::env;

/// Stores global information about the tool installation process and detailed
/// info about installing each particular tool.
/// 
/// This data type is parsed from the TOML configuration file.
#[derive(Debug)]
pub struct Config {
  /// Directory to store all locally downloaded tools
  pub store_directory: PathBuf,

  /// Info about each individual tool
  pub tools: HashMap<String, ConfigAsset>,
}

/// Additional details, telling how to download a tool
#[derive(Debug)]
pub struct ConfigAsset {
    /// GitHub repository author
    pub owner: Option<String>,

    /// GitHub repository name
    pub repo: Option<String>,

    /// Executable name inside the .tar.gz or .zip archive
    /// Defaults to `repo` if not specified
    pub exe_name: Option<String>,

    /// Name of the specific asset to download
    pub asset_name: AssetName,
}

/// All info about installing a tool from GitHub releases
#[derive(Debug)]
pub struct ToolInfo {
    /// GitHub repository author
    pub owner: String,

    /// GitHub repository name
    pub repo: String,

    /// Executable name inside the .tar.gz or .zip archive
    pub exe_name: String,

    /// Asset name depending on the OS
    pub asset_name: AssetName,
}

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

pub enum ToolConfigError {
   Unknown(String), 
}

pub enum Tool {
    Known(ToolInfo),
    Error(ToolConfigError), 
}

pub fn resolve_tool(tool_name: &str, tool_config: &ConfigAsset) -> Tool {
    match tool_name {
        "ripgrep" => Tool::Known(ToolInfo {
            owner: "BurntSushi".to_string(),
            repo: "ripgrep".to_string(),
            exe_name: "rg".to_string(),
            asset_name: AssetName {
                linux: Some("unknown-linux-musl".to_string()), 
                macos: Some("apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
              }
        }),
        other => Tool::Error(ToolConfigError::Unknown(String::from("Unknown tool"))),
    }
}