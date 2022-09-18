use std::env;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssetError {
    /// Asset name of this OS is unknown
    NameUnknown,

    /// Asset name is not in the fetched assets
    NotFound(String),
}

impl Display for AssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameUnknown => {
                write!(
                    f,
                    "Unknown asset selector for OS: {}. Specify 'asset_name.your_os' in the cofig.",
                    env::consts::OS
                )
            }
            Self::NotFound(asset_name) => {
                write!(f, "No asset matching name: {}", asset_name)
            }
        }
    }
}
