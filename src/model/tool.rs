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

impl ToolInfoTag {
    pub fn as_specific_tag(&self) -> Option<&str> {
        match self {
            Self::Latest => None,
            Self::Specific(version) => Some(&version),
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

    /// Asset name depending on the OS
    pub asset_name: AssetName,

    /// Version tag
    pub tag: ToolInfoTag,
}
