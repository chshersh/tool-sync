use std::env;
use std::error::Error;
use std::io::Read;

use crate::model::release::{Asset, Release};

/// GitHub API client to handle all API requests
#[derive(Debug)]
pub struct Client {
    pub owner: String,
    pub repo: String,
    pub version: String,

    pub proxy: Option<ureq::Proxy>,
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

    pub fn fetch_release_info(&self) -> Result<Release, Box<dyn Error>> {
        let release_url = self.release_url();
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .proxy(self.proxy.clone())
            .build()
            .into();
        let req = agent
            .get(release_url)
            .header("accept", "application/vnd.github+json")
            .header("user-agent", "chshersh/tool-sync-0.2.0");
        let req = add_auth_header(req);
        let json = req.call()?.into_body().read_to_string()?;
        let release: Release = facet_json::from_str(&json).map_err(|jpewc| todo!("{jpewc}"))?;
        Ok(release)
    }

    pub fn get_asset_stream(
        &self,
        asset: &Asset,
    ) -> Result<Box<dyn Read + Send + Sync>, Box<ureq::Error>> {
        let asset_url = self.asset_url(asset.id);
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .proxy(self.proxy.clone())
            .build()
            .into();
        let req = agent
            .get(asset_url)
            .header("accept", "application/octet-stream")
            .header("user-agent", "chshersh/tool-sync-0.2.0");
        let req = add_auth_header(req);

        Ok(Box::new(req.call()?.into_body().into_reader()))
    }
}

fn add_auth_header<B>(req: ureq::RequestBuilder<B>) -> ureq::RequestBuilder<B> {
    match env::var("GITHUB_TOKEN") {
        Err(_) => req,
        Ok(token) => req.header("authorization", &format!("token {token}")),
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
            proxy: None,
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
            proxy: None,
        };

        assert_eq!(
            client.release_url(),
            "https://api.github.com/repos/OWNER/REPO/releases/tags/SPECIFIC_TAG"
        );
    }
}
