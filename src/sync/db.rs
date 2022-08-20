use crate::model::tool::ToolInfo;
use crate::model::asset_name::AssetName;

/// Get info about known tools from a hardcoded database
pub fn lookup_tool(tool_name: &str) -> Option<ToolInfo> {
    match tool_name {
        "exa" => Some(ToolInfo {
            owner: "ogham".to_string(),
            repo: "exa".to_string(),
            exe_name: "exa".to_string(),
            asset_name: AssetName {
                linux: Some("linux-x86_64-musl".to_string()), 
                macos: Some("macos-x86_64".to_string()), 
                windows: None,
              }
        }),
        "ripgrep" => Some(ToolInfo {
            owner: "BurntSushi".to_string(),
            repo: "ripgrep".to_string(),
            exe_name: "rg".to_string(),
            asset_name: AssetName {
                linux: Some("unknown-linux-musl".to_string()), 
                macos: Some("apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
              }
        }),
        _ => None,
    }
}
