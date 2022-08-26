use crate::model::asset_name::AssetName;

#[derive(Debug, PartialEq)]
pub enum Tool {
    Known(ToolInfo),
    Error(ToolError), 
}

#[derive(Debug, PartialEq)]
pub enum ToolError {
    /// Probably a known tool but specified differently. E.g. 'rg' instead of 'ripgrep'
    Suggestion {
        perhaps: String,
    },

    /// Not enough configuration to install the tool
    Invalid, 
}

impl ToolError {
    pub fn display(&self) -> String {
        match self {
            ToolError::Suggestion {perhaps} =>
                format!("[suggestion] Perhaps you meant: '{}'?", perhaps),
            ToolError::Invalid =>
                format!("[error] Not detailed enough configuration)"),
        }
    }
}

/// All info about installing a tool from GitHub releases
#[derive(Debug, PartialEq)]
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