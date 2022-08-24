use crate::model::asset_name::AssetName;

pub enum Tool {
    Known(ToolInfo),
    Error(ToolError), 
}

pub enum ToolError {
    /// Probably a known tool but specified differently. E.g. 'rg' instead of 'ripgrep'
    Suggestion {
        provided: String,
        perhaps: String,
    },

    /// Not enough configuration to install the tool
    Invalid(String), 
}

impl ToolError {
    pub fn display(&self) -> String {
        match self {
            ToolError::Suggestion {provided, perhaps} =>
                format!("[error] Incorrect tool name '{}'. Perhaps you meant: '{}'?", provided, perhaps),
            ToolError::Invalid(tool) =>
                format!("[error] Not enough configuration to sync: '{}'", tool),
        }
    }
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