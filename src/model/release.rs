use serde::Deserialize;
use std::fmt::{Display, Formatter, Write};
use regex;

use crate::infra::err;
use crate::model::os::get_current_os;

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

//#[derive(Debug, PartialEq, Eq)]
#[derive(Debug)]
pub enum AssetError {
    /// Asset name of this OS is unknown
    OsSelectorUnknown,

    /// Asset pattern is invalid
    InvalidPattern(regex::Error),

    /// Asset name is not in the fetched assets
    NotFound(String),

    /// Multiple asset names are found
    MultipleFound(Vec<String>),
}

impl Display for AssetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OsSelectorUnknown => {
                write!(
                    f,
                    "Unknown asset selector for the current OS. Specify 'asset_name.{}' in the config.",
                    get_current_os()
                )
            }
            Self::InvalidPattern(regex_err) => {
                write!(f, "Invalid asset pattern: {}", regex_err)
            }
            Self::NotFound(asset_name) => {
                write!(f, "No asset matching name: {}", asset_name)
            }
            Self::MultipleFound(assets) => {
                let mut formatted: String = String::from("\n");
                for asset in assets {
                    if let Err(e) = writeln!(formatted, "\t * {}", asset) {
                        err::abort_suggest_issue(e)
                    };
                }
                write!(
                    f,
                    "\nMultiple name matches found for this asset:\n{}\nPlease add one of these to the config.",
                    formatted
                )
            }
        }
    }
}
