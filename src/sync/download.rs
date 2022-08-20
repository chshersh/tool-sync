use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use ureq;

use crate::model::release::{Release, Asset};

pub struct Downloader<'a> {
    pub owner: &'a str,
    pub repo: &'a str,
    pub asset_name: &'a str,
}

/// Info about the downloaded asset
pub struct DownloadInfo {
    pub archive_path: PathBuf,
    pub asset_name: String,
}

impl<'a> Downloader<'a> {
    fn release_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{owner}/{repo}/releases/latest",
            owner = self.owner,
            repo = self.repo
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

    fn download_release(&self) -> Result<Release, Box<dyn Error>> {
        let release_url = self.release_url();
        println!("Release URL: {}", release_url);

        let release: Release =
            ureq::get(&release_url)
            .set("Accept", "application/vnd.github+json")
            .set("User-Agent", "chshersh/tool-sync-1.0")
            .call()?
            .into_json()?;

        Ok(release)
    }

    fn download_asset(&self, tmp_dir: &Path, asset: &Asset) -> Result<PathBuf, Box<dyn Error>> {
        let asset_url = self.asset_url(asset.id);
        println!("Asset URL: {}", asset_url);
        
        let mut stream = ureq::get(&asset_url)
            .set("Accept", "application/octet-stream")
            .set("User-Agent", "chshersh/tool-sync-1.0")
            .call()?
            .into_reader();
        
        let download_path = tmp_dir.join(&asset.name);
        let mut destination = File::create(&download_path)?;

        let mut buffer = [0; 4096];
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
        
            destination.write(&buffer[..bytes_read])?;
        }

        Ok(download_path)

    }

    /// Download an asset and return a path of the downloaded artefact
    pub fn download(&self, tmp_dir: &Path) -> Result<DownloadInfo, Box<dyn Error>> {
        let release = self.download_release()?;

        println!("{:#?}", release);

        let asset = release
                .assets
                .iter()
                .find(|&asset| asset.name.contains(self.asset_name));

        match asset {
            None => Err(format!("No asset matching name: {}", self.asset_name).into()),
            Some(asset) => {
                let archive_path = self.download_asset(tmp_dir, asset)?;

                Ok(DownloadInfo{
                    archive_path,
                    asset_name: asset.name.clone(),
                })
            }
        }
    }
}