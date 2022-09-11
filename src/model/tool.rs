use super::release::Asset;
use crate::infra::client::Client;
use crate::model::asset_name::AssetName;

#[derive(Debug, PartialEq, Eq)]
pub enum Tool {
    Known(ToolInfo),
    Error(ToolError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ToolError {
    /// Probably a known tool but specified differently. E.g. 'rg' instead of 'ripgrep'
    Suggestion { perhaps: String },

    /// Not enough configuration to install the tool
    Invalid,
}

impl ToolError {
    pub fn display(&self) -> String {
        match self {
            ToolError::Suggestion { perhaps } => {
                format!("[suggestion] Perhaps you meant: '{}'?", perhaps)
            }
            ToolError::Invalid => "[error] Not detailed enough configuration)".to_string(),
        }
    }
}

/// Determines whether to download the latest version of a tool or a
/// specific version of it.
#[derive(Debug, PartialEq, Eq)]
pub enum ToolInfoTag {
    /// Download latest
    Latest,
    /// Download a specific version
    Specific(String),
}

const LATEST_VERSION: &str = "latest";

impl ToolInfoTag {
    pub fn to_str_version(&self) -> String {
        match self {
            Self::Latest => LATEST_VERSION.to_owned(),
            Self::Specific(version) => format!("tags/{}", version),
        }
    }
}

/// All info about installing a tool from GitHub releases
#[derive(Debug, PartialEq, Eq)]
pub struct ToolInfo {
    /// GitHub repository author
    pub owner: String,

    /// GitHub repository name
    pub repo: String,

    /// Executable name inside the .tar.gz or .zip archive
    pub exe_name: String,

    /// Version tag
    pub tag: ToolInfoTag,

    /// Asset name depending on the OS
    pub asset_name: AssetName,
}

impl ToolInfo {
    pub fn select_asset(&self, assets: &[Asset]) -> Result<Asset, String> {
        match self.asset_name.get_name_by_os() {
            None => Err(String::from(
                "Don't know the asset name for this OS: specify it explicitly in the config",
            )),
            Some(asset_name) => {
                let asset = assets.iter().find(|&asset| asset.name.contains(asset_name));

                match asset {
                    None => Err(format!("No asset matching name: {}", asset_name)),
                    Some(asset) => Ok(asset.clone()),
                }
            }
        }
    }
}

/// All information about the tool, needed to download its asset after fetching
/// the release and asset info. Values of this type are created in
/// `src/sync/prefetch.rs` from `ToolInfo`.
pub struct ToolAsset {
    /// Name of the tool (e.g. "ripgrep" or "exa")
    pub tool_name: String,

    /// Specific git tag (e.g. "v3.4.2")
    /// This value is the result of `ToolInfoTag::to_str_version` so "latest"
    /// **can't** be here.
    pub tag: String,

    /// Executable name inside the .tar.gz or .zip archive
    pub exe_name: String,

    /// The selected asset
    pub asset: Asset,

    /// GitHub API client that produces the stream for downloading the asset
    pub client: Client,
}
