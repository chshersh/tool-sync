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
            exe_name: config_asset.exe_name.clone().unwrap_or(self.exe_name.clone()),
            asset_name: AssetName {
                linux: config_asset.asset_name.linux.clone().or(self.asset_name.linux.clone()),
                macos: config_asset.asset_name.macos.clone().or(self.asset_name.macos.clone()),
                windows: config_asset.asset_name.windows.clone().or(self.asset_name.windows.clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_tool_with_empty_config_asset() {
        let tool_name = "ripgrep";

        let config_asset = ConfigAsset {
                owner: None,
                repo: None,
                exe_name: None,
                asset_name: AssetName { 
                    linux: None, 
                    macos: None, 
                    windows: None 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Known(lookup_tool(tool_name).unwrap())
        );
    }

    #[test]
    fn unknown_tool_with_empty_config_asset() {
        let tool_name = "abcdef";

        let config_asset = ConfigAsset {
                owner: None,
                repo: None,
                exe_name: None,
                asset_name: AssetName { 
                    linux: None, 
                    macos: None, 
                    windows: None 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Error(ToolError::Invalid(tool_name.to_owned()))
        );
    }

    #[test]
    fn wrong_tool_with_empty_config_asset() {
        let tool_name = "rg";

        let config_asset = ConfigAsset {
                owner: None,
                repo: None,
                exe_name: None,
                asset_name: AssetName { 
                    linux: None, 
                    macos: None, 
                    windows: None 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Error(ToolError::Suggestion {
                provided: tool_name.to_owned(),
                perhaps: "ripgrep".to_owned(),
            })
        );
    }

    #[test]
    fn partial_configuration() {
        let tool_name = "abcdef";

        let config_asset = ConfigAsset {
                owner: Some(String::from("chshersh")),
                repo: None,
                exe_name: Some(String::from("abcdefu")),
                asset_name: AssetName { 
                    linux: None, 
                    macos: None, 
                    windows: None 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Error(ToolError::Invalid(tool_name.to_owned()))
        );
    }

    #[test]
    fn partial_override() {
        let tool_name = "ripgrep";

        let config_asset = ConfigAsset {
                owner: Some(String::from("chshersh")),
                repo: None,
                exe_name: Some(String::from("abcdefu")),
                asset_name: AssetName { 
                    linux: None, 
                    macos: None, 
                    windows: None 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Known(ToolInfo {
                owner: "chshersh".to_string(),
                repo: "ripgrep".to_string(),
                exe_name: "abcdefu".to_string(),
                asset_name: AssetName {
                    linux: Some("unknown-linux-musl".to_string()), 
                    macos: Some("apple-darwin".to_string()), 
                    windows: Some("x86_64-pc-windows-msvc".to_string()),
                }
            })
        );
    }

    #[test]
    fn full_override() {
        let tool_name = "ripgrep";

        let config_asset = ConfigAsset {
                owner: Some(String::from("chshersh")),
                repo: Some(String::from("Pluto")),
                exe_name: Some(String::from("abcdefu")),
                asset_name: AssetName { 
                    linux: Some(String::from("my-linux")), 
                    macos: Some(String::from("my-macos")), 
                    windows: Some(String::from("yours-windows")) 
                }
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset), 
            Tool::Known(ToolInfo {
                owner: "chshersh".to_string(),
                repo: "Pluto".to_string(),
                exe_name: "abcdefu".to_string(),
                asset_name: AssetName {
                    linux: Some("my-linux".to_string()), 
                    macos: Some("my-macos".to_string()), 
                    windows: Some("yours-windows".to_string()),
                }
            })
        );
    }

}