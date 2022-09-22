use shellexpand;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::err;
use crate::model::asset_name::AssetName;
use crate::model::tool::{ToolInfo, ToolInfoTag};

/// Stores global information about the tool installation process and detailed
/// info about installing each particular tool.
///
/// This data type is parsed from the TOML configuration file.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Config {
    /// Directory to store all locally downloaded tools
    pub store_directory: String,

    /// Info about each individual tool
    pub tools: BTreeMap<String, ConfigAsset>,
}

/// Additional details, telling how to download a tool
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConfigAsset {
    /// GitHub repository author
    pub owner: Option<String>,

    /// GitHub repository name
    pub repo: Option<String>,

    /// Executable name inside the .tar.gz or .zip archive
    /// Defaults to `repo` if not specified
    pub exe_name: Option<String>,

    /// Release tag to download
    /// Defaults to the latest release
    pub tag: Option<String>,

    /// Name of the specific asset to download
    pub asset_name: AssetName,
}

impl From<ToolInfo> for ConfigAsset {
    fn from(tool_info: ToolInfo) -> Self {
        let tag = match tool_info.tag {
            ToolInfoTag::Specific(version) => Some(version),
            ToolInfoTag::Latest => None,
        };

        Self {
            owner: Some(tool_info.owner),
            repo: Some(tool_info.repo),
            exe_name: Some(tool_info.exe_name),
            tag,
            asset_name: tool_info.asset_name,
        }
    }
}

impl Config {
    /// Shellexpands store directory, check whether it exists and exits with
    /// error if 'store_directory' doesn't exist
    pub fn ensure_store_directory(&self) -> PathBuf {
        let expanded_store_directory = shellexpand::full(&self.store_directory);

        let store_directory = match expanded_store_directory {
            Err(e) => err::abort_with(e),
            Ok(cow_path) => PathBuf::from(cow_path.into_owned()),
        };

        let has_store_directory = store_directory.as_path().is_dir();

        if !has_store_directory {
            err::abort_with(format!(
                "Specified directory for storing tools doesn't exist: {}",
                store_directory.display()
            ));
        }

        store_directory
    }
}
