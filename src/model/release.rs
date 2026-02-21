use facet_pretty::FacetPretty as _;
use std::fmt::{Display, Formatter};

use crate::model::os::get_current_os;

#[derive(facet::Facet, Debug)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(facet::Facet, Debug, Clone, Eq, PartialEq)]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub size: u64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssetError {
    /// Asset name of this OS is unknown
    OsSelectorUnknown,

    /// Asset name is not in the fetched assets
    NotFound(String, Vec<String>),

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
            Self::NotFound(asset_name, available_assets) => {
                write!(
                    f,
                    "No asset matching name: {asset_name} among {}",
                    available_assets.pretty()
                )
            }
            Self::MultipleFound(assets) => {
                write!(
                    f,
                    "\nMultiple name matches found for this asset:\n{}\nPlease add one of these to the config.",
                    assets.pretty()
                )
            }
        }
    }
}
