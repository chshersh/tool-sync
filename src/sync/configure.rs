use crate::config::schema::ConfigAsset;
use crate::model::tool::{Tool, ToolError, ToolInfo};
use crate::model::asset_name::AssetName;
use crate::sync::db::lookup_tool;

pub fn configure_tool(tool_name: &str, config_asset: &ConfigAsset) -> Tool {
    match lookup_tool(tool_name) {
        // This is a known tool; we get hardcoded info and update it with config
        Some(tool_info) => Tool::Known(tool_info.configure(config_asset)),
        
        // Unknown tool: try to fully configure from scratch
        None => match full_configure(config_asset) {
            // No need to call '.configure' here
            Some(tool_info) => Tool::Known(tool_info),
            
            // Not enough configuration: suggestion with error messages
            None => match tool_name {
                "rg" => Tool::Error(ToolError::Suggestion { 
                    provided: "rg".to_string(),
                    perhaps: "ripgrep".to_string(),
                }),
                "difft" => Tool::Error(ToolError::Suggestion { 
                    provided: "difft".to_string(),
                    perhaps: "difftastic".to_string(),
                }),
                other => Tool::Error(ToolError::Invalid(other.to_string())),
            }
        },
    }
}

/// Configure 'ToolInfo' completely from 'ConfigAsset'
fn full_configure(config_asset: &ConfigAsset) -> Option<ToolInfo> {
    let owner = config_asset.owner.clone()?;
    let repo = config_asset.repo.clone()?;
    let exe_name = config_asset.exe_name.clone()?;

    Some(ToolInfo {
        owner,
        repo,
        exe_name,
        asset_name: AssetName {
            linux: config_asset.asset_name.linux.clone(),
            macos: config_asset.asset_name.macos.clone(),
            windows: config_asset.asset_name.windows.clone(),
        }
    })
}

impl ToolInfo {
    /// Update hardcoded tool info with configuration from TOML
    pub fn configure(&self, config_asset: &ConfigAsset) -> ToolInfo {
        ToolInfo {
            owner: config_asset.owner.clone().unwrap_or(self.owner.clone()),
            repo: config_asset.repo.clone().unwrap_or(self.repo.clone()),
            exe_name: config_asset.repo.clone().unwrap_or(self.exe_name.clone()),
            asset_name: AssetName {
                linux: config_asset.asset_name.linux.clone().or(self.asset_name.linux.clone()),
                macos: config_asset.asset_name.macos.clone().or(self.asset_name.macos.clone()),
                windows: config_asset.asset_name.windows.clone().or(self.asset_name.windows.clone()),
            }
        }
    }
}