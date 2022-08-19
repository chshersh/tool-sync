use std::collections::HashMap;
use std::path::PathBuf;

use crate::model::asset_name::AssetName;

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
