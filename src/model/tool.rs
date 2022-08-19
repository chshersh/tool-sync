use crate::config::schema::ConfigAsset;
use crate::model::asset_name::AssetName;

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