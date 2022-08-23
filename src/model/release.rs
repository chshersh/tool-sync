use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    // tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: u32,
    pub name: String,
    pub size: u64,
}
