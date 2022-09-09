use crate::config::schema::ConfigAsset;
use crate::model::asset_name::AssetName;
use crate::model::tool::{Tool, ToolError, ToolInfo, ToolInfoTag};
use crate::sync::db::lookup_tool;
use serde::Deserialize;

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
                    perhaps: "ripgrep".to_string(),
                }),
                "difft" => Tool::Error(ToolError::Suggestion {
                    perhaps: "difftastic".to_string(),
                }),
                _other => Tool::Error(ToolError::Invalid),
            },
        },
    }
}

#[derive(Deserialize)]
pub struct RepoInformation {
    pub name: String,
    // There are other fields too, but we don't need them now
    // They may be added later if needed at all
}

fn get_exe_name(owner: String, repo: String) -> Result<String, ureq::Error> {
    let url = format!(
        "https://api.github.com/repos/{owner}/{repo}",
        owner = owner,
        repo = repo
    );
    let repo: RepoInformation = ureq::get(&url).call()?.into_json()?;
    Ok(repo.name)
}

/// Configure 'ToolInfo' completely from 'ConfigAsset'
fn full_configure(config_asset: &ConfigAsset) -> Option<ToolInfo> {
    let owner = config_asset.owner.clone()?;
    let repo = config_asset.repo.clone()?;
    let exe_name = match config_asset.exe_name.clone() {
        Some(exe_name) => exe_name,
        None => get_exe_name(owner.clone(), repo.clone()).unwrap(),
    };
    let tag = config_asset
        .tag
        .clone()
        .map(ToolInfoTag::Specific)
        .unwrap_or(ToolInfoTag::Latest);

    Some(ToolInfo {
        owner,
        repo,
        exe_name,
        asset_name: AssetName {
            linux: config_asset.asset_name.linux.clone(),
            macos: config_asset.asset_name.macos.clone(),
            windows: config_asset.asset_name.windows.clone(),
        },
        tag,
    })
}

impl ToolInfo {
    /// Update hardcoded tool info with configuration from TOML
    pub fn configure(&self, config_asset: &ConfigAsset) -> ToolInfo {
        ToolInfo {
            owner: config_asset
                .owner
                .clone()
                .unwrap_or_else(|| self.owner.clone()),
            repo: config_asset
                .repo
                .clone()
                .unwrap_or_else(|| self.repo.clone()),
            exe_name: config_asset
                .exe_name
                .clone()
                .unwrap_or_else(|| self.exe_name.clone()),
            asset_name: AssetName {
                linux: config_asset
                    .asset_name
                    .linux
                    .clone()
                    .or_else(|| self.asset_name.linux.clone()),
                macos: config_asset
                    .asset_name
                    .macos
                    .clone()
                    .or_else(|| self.asset_name.macos.clone()),
                windows: config_asset
                    .asset_name
                    .windows
                    .clone()
                    .or_else(|| self.asset_name.windows.clone()),
            },
            tag: config_asset
                .tag
                .clone()
                .map(|version| ToolInfoTag::Specific(version))
                .unwrap_or(ToolInfoTag::Latest),
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
                windows: None,
            },
            tag: None,
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
                windows: None,
            },
            tag: None,
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset),
            Tool::Error(ToolError::Invalid)
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
                windows: None,
            },
            tag: None,
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset),
            Tool::Error(ToolError::Suggestion {
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
                windows: None,
            },
            tag: Some(String::from("1.2.3")),
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset),
            Tool::Error(ToolError::Invalid)
        );
    }

    #[test]
    fn full_configuration() {
        let tool_name = "abcdef";

        let config_asset = ConfigAsset {
            owner: Some(String::from("chshersh")),
            repo: Some(String::from("Pluto")),
            exe_name: Some(String::from("abcdefu")),
            asset_name: AssetName {
                linux: Some(String::from("my-linux")),
                macos: Some(String::from("my-macos")),
                windows: Some(String::from("yours-windows")),
            },
            tag: Some(String::from("1.2.3")),
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
                },
                tag: ToolInfoTag::Specific("1.2.3".to_string()),
            })
        );
    }

    #[test]
    fn get_exe_name() {
        let tool_name = "abcdef";

        let config_asset = ConfigAsset {
            owner: Some(String::from("chshersh")),
            repo: Some(String::from("tool-sync")),
            exe_name: None,
            asset_name: AssetName {
                linux: Some(String::from("my-linux")),
                macos: Some(String::from("my-macos")),
                windows: Some(String::from("yours-windows")),
            },
            tag: Some(String::from("1.0.0")),
        };

        assert_eq!(
            configure_tool(tool_name, &config_asset),
            Tool::Known(ToolInfo {
                owner: "chshersh".to_string(),
                repo: "tool-sync".to_string(),
                exe_name: "tool-sync".to_string(),
                asset_name: AssetName {
                    linux: Some("my-linux".to_string()),
                    macos: Some("my-macos".to_string()),
                    windows: Some("yours-windows".to_string()),
                },
                tag: ToolInfoTag::Specific("1.0.0".to_string()),
            })
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
                windows: None,
            },
            tag: None,
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
                },
                tag: ToolInfoTag::Latest,
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
                windows: Some(String::from("yours-windows")),
            },
            tag: Some(String::from("3.2.1")),
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
                },
                tag: ToolInfoTag::Specific("3.2.1".to_string()),
            })
        );
    }
}
