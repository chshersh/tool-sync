use indicatif::ProgressBar;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::infra::client::Client;
use crate::model::release::Asset;
use crate::sync::progress::SyncProgress;

pub struct Downloader<'a> {
    pub asset: &'a Asset,
    pub client: &'a Client,
    pub pb_msg: &'a ProgressBar,
    pub sync_progress: &'a SyncProgress,
}

/// Info about the downloaded asset
pub struct DownloadInfo {
    pub archive_path: PathBuf,
}

impl<'a> Downloader<'a> {
    fn download_asset(&self, tmp_dir: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let mut stream = self.client.get_asset_stream(self.asset)?;

        let download_path = tmp_dir.join(&self.asset.name);
        let mut destination = File::create(&download_path)?;

        self.pb_msg.set_message("Downloading...");
        let pb_downloading = self.sync_progress.create_progress_bar(self.asset.size);

        let mut buffer = [0; 4096];
        while let Ok(bytes_read) = stream.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            pb_downloading.inc(bytes_read as u64);
            destination.write_all(&buffer[..bytes_read])?;
        }

        self.pb_msg.set_message("Downloaded!");
        SyncProgress::finish_progress(pb_downloading);

        Ok(download_path)
    }

    /// Download an asset and return a path of the downloaded artefact
    pub fn download(&self, tmp_dir: &Path) -> Result<DownloadInfo, Box<dyn Error>> {
        self.pb_msg.set_message("Fetching info...");

        let archive_path = self.download_asset(tmp_dir)?;

        Ok(DownloadInfo { archive_path })
    }
}
