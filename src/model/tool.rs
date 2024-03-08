use super::release::Asset;
use crate::infra::client::Client;
use crate::model::asset_name::AssetName;
use crate::model::release::AssetError;
use std::fmt::{Display, Formatter};

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

impl Display for ToolError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::Suggestion { perhaps } => {
                write!(f, "[suggestion] Perhaps you meant: '{}'?", perhaps)
            }
            ToolError::Invalid => write!(f, "[error] Not detailed enough configuration"),
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
const COMPANION_EXTENSIONS: [&str; 11] = [
    ".asc",
    ".md5",
    ".md5sum",
    ".sha1",
    ".sha1sum",
    ".sha256",
    ".sha256sum",
    ".sha512",
    ".sha512sum",
    ".zsync",
    ".sbom",
];

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
    /// Select an Asset from all Assets based on which Operating System is used
    pub fn select_asset(&self, assets: &[Asset]) -> Result<Asset, AssetError> {
        match self.asset_name.get_name_by_os() {
            None => Err(AssetError::OsSelectorUnknown),
            Some(asset_name) => {
                let mut filtered_assets = assets
                    .iter()
                    .filter(|&asset| {
                        !COMPANION_EXTENSIONS
                            .iter()
                            .any(|ext| asset.name.ends_with(ext))
                    })
                    .filter(|&asset| asset.name.contains(asset_name))
                    .map(|asset| asset.to_owned())
                    .collect::<Vec<Asset>>();
                match filtered_assets.len() {
                    0 => Err(AssetError::NotFound(asset_name.clone())),

                    // This is safe because there is exactly 1 element
                    1 => Ok(filtered_assets.remove(0)),
                    _ => {
                        let assets: Vec<String> =
                            filtered_assets.into_iter().map(|item| item.name).collect();
                        Err(AssetError::MultipleFound(assets))
                    }
                }
            }
        }
    }
}

/// All information about the tool, needed to download its asset after fetching
/// the release and asset info. Values of this type are created in
/// `src/sync/prefetch.rs` from `ToolInfo`.
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_found() {
        let asset_name = "asset";

        let tool_info = ToolInfo {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            exe_name: "exe".to_string(),
            tag: ToolInfoTag::Latest,
            asset_name: AssetName {
                linux: Some(asset_name.to_string()),
                macos: Some(asset_name.to_string()),
                windows: Some(asset_name.to_string()),
            },
        };

        let assets = vec![
            Asset {
                id: 1,
                name: "1".to_string(),
                size: 10,
            },
            Asset {
                id: 2,
                name: asset_name.to_string(),
                size: 50,
            },
            Asset {
                id: 3,
                name: "3".to_string(),
                size: 77,
            },
        ];

        assert_eq!(
            tool_info.select_asset(&assets),
            Ok(Asset {
                id: 2,
                name: asset_name.to_string(),
                size: 50
            })
        );
    }

    #[test]
    fn multiple_asset_found() {
        let asset_name = "asset";

        let tool_info = ToolInfo {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            exe_name: "exe".to_string(),
            tag: ToolInfoTag::Latest,
            asset_name: AssetName {
                linux: Some(asset_name.to_string()),
                macos: Some(asset_name.to_string()),
                windows: Some(asset_name.to_string()),
            },
        };

        let assets = vec![
            Asset {
                id: 1,
                name: "asset_1".to_string(),
                size: 10,
            },
            Asset {
                id: 2,
                name: "asset_2".to_string(),
                size: 50,
            },
            Asset {
                id: 3,
                name: "asset_3".to_string(),
                size: 77,
            },
            Asset {
                id: 3,
                name: "asset_3.asc".to_string(),
                size: 12,
            },
            Asset {
                id: 3,
                name: "not a match".to_string(),
                size: 77,
            },
        ];

        assert_eq!(
            tool_info.select_asset(&assets),
            Err(AssetError::MultipleFound(vec![
                "asset_1".into(),
                "asset_2".into(),
                "asset_3".into()
            ]))
        );
    }

    #[test]
    fn asset_not_found() {
        let asset_name = "asset";

        let tool_info = ToolInfo {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            exe_name: "exe".to_string(),
            tag: ToolInfoTag::Latest,
            asset_name: AssetName {
                linux: Some(asset_name.to_string()),
                macos: Some(asset_name.to_string()),
                windows: Some(asset_name.to_string()),
            },
        };

        let assets = vec![
            Asset {
                id: 1,
                name: "1".to_string(),
                size: 10,
            },
            Asset {
                id: 2,
                name: "2".to_string(),
                size: 50,
            },
            Asset {
                id: 3,
                name: "3".to_string(),
                size: 77,
            },
        ];

        assert_eq!(
            tool_info.select_asset(&assets),
            Err(AssetError::NotFound(asset_name.to_string()))
        );
    }

    #[test]
    fn asset_os_selector_unknown() {
        let tool_info = ToolInfo {
            owner: "owner".to_string(),
            repo: "repo".to_string(),
            exe_name: "exe".to_string(),
            tag: ToolInfoTag::Latest,
            asset_name: AssetName {
                linux: None,
                macos: None,
                windows: None,
            },
        };

        assert_eq!(
            tool_info.select_asset(&[]),
            Err(AssetError::OsSelectorUnknown)
        );
    }
}
