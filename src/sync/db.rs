use crate::model::tool::ToolInfo;
use crate::model::asset_name::AssetName;

/// Get info about known tools from a hardcoded database
pub fn lookup_tool(tool_name: &str) -> Option<ToolInfo> {
    match tool_name {
        "bat" => Some(ToolInfo {
            owner: "sharkdp".to_string(),
            repo: "bat".to_string(),
            exe_name: "bat".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-musl".to_string()), 
                macos: Some("x86_64-apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
              }
        }),
        "difftastic" => Some(ToolInfo {
            owner: "Wilfred".to_string(),
            repo: "difftastic".to_string(),
            exe_name: "difft".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-gnu".to_string()), 
                macos: Some("x86_64-apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
              }
        }),
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
        "fd" => Some(ToolInfo {
            owner: "sharkdp".to_string(),
            repo: "fd".to_string(),
            exe_name: "fd".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-musl".to_string()), 
                macos: Some("x86_64-apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
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
        "tokei" => Some(ToolInfo {
            owner: "XAMPPRocky".to_string(),
            repo: "tokei".to_string(),
            exe_name: "tokei".to_string(),
            asset_name: AssetName {
                linux: Some("x86_64-unknown-linux-musl".to_string()), 
                macos: Some("apple-darwin".to_string()), 
                windows: Some("x86_64-pc-windows-msvc".to_string()),
              }
        }),
        _ => None,
    }
}
