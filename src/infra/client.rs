use std::env;
use std::error::Error;
use std::io::Read;

use crate::model::release::{Asset, Release};
use crate::model::tag::Tag;

/// GitHub API client to handle all API requests
pub struct Client {
    pub owner: String,
    pub repo: String,
    pub version: String,
}

impl Client {
    fn release_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/{version}",
            owner = self.owner,
            repo = self.repo,
            version = self.version,
        )
    }

    fn asset_url(&self, asset_id: u32) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/assets/{asset_id}",
            owner = self.owner,
            repo = self.repo,
            asset_id = asset_id
        )
    }

    fn tags_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/tags",
            owner = self.owner,
            repo = self.repo,
        )
    }

    pub fn fetch_release_info(&self) -> Result<Release, Box<dyn Error>> {
        let release_url = self.release_url();

        let req = add_auth_header(
            ureq::get(&release_url)
                .set("Accept", "application/vnd.github+json")
                .set("User-Agent", "chshersh/tool-sync-0.2.0"),
        );

        let release: Release = req.call()?.into_json()?;

        Ok(release)
    }

    pub fn fetch_available_tags(&self) -> Result<Vec<Tag>, Box<dyn Error>> {
        let tags_url = self.tags_url();

        let req = add_auth_header(
            ureq::get(&tags_url)
                .set("Accept", "application/vnd.github+json")
                .set("User-Agent", "chshersh/tool-sync-0.2.0"),
        );

        let release: Vec<Tag> = req.call()?.into_json()?;

        Ok(release)
    }

    pub fn get_asset_stream(
        &self,
        asset: &Asset,
    ) -> Result<Box<dyn Read + Send + Sync>, ureq::Error> {
        let asset_url = self.asset_url(asset.id);

        let req = add_auth_header(
            ureq::get(&asset_url)
                .set("Accept", "application/octet-stream")
                .set("User-Agent", "chshersh/tool-sync-0.2.0"),
        );

        Ok(req.call()?.into_reader())
    }
}

fn add_auth_header(req: ureq::Request) -> ureq::Request {
    match env::var("GITHUB_TOKEN") {
        Err(_) => req,
        Ok(token) => req.set("Authorization", &format!("token {}", token)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::model::tool::ToolInfoTag;

    #[test]
    fn release_url_with_latest_tag_is_correct() {
        let client = Client {
            owner: String::from("OWNER"),
            repo: String::from("REPO"),
            version: ToolInfoTag::Latest.to_str_version(),
        };

        assert_eq!(
            client.release_url(),
            "https://api.github.com/repos/OWNER/REPO/releases/latest"
        );
    }

    #[test]
    fn release_url_with_specific_tag_is_correct() {
        let client = Client {
            owner: String::from("OWNER"),
            repo: String::from("REPO"),
            version: ToolInfoTag::Specific(String::from("SPECIFIC_TAG")).to_str_version(),
        };

        assert_eq!(
            client.release_url(),
            "https://api.github.com/repos/OWNER/REPO/releases/tags/SPECIFIC_TAG"
        );
    }
}
