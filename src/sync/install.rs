use indicatif::ProgressBar;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use tempdir::TempDir;

use crate::config::schema::ConfigAsset;
use crate::model::tool::{Tool, ToolInfo};

use super::archive::Archive;
use super::download::Downloader;
use super::configure::configure_tool;
use super::progress::SyncProgress;

pub struct Installer {
    store_directory: PathBuf,
    tmp_dir: TempDir,
    sync_progress: SyncProgress,
}

impl Installer {
    /// This functions panics when it can't create a temporary directory
    /// (e.g. not enough disk space?)
    pub fn mk(store_directory: PathBuf, sync_progress: SyncProgress) -> Installer {
        let tmp_dir = TempDir::new("tool-sync");
        match tmp_dir {
            Err(e) => {
                eprintln!("Error creating temporary directory: {}", e);
                process::exit(1);
            },
            Ok(tmp_dir) => Installer
                { store_directory
                , tmp_dir
                , sync_progress
                }
        }
    }

    pub fn install(&self, tool_name: &str, config_asset: &ConfigAsset) {
        let pb_msg = self.sync_progress.create_message_bar(tool_name);

        match configure_tool(tool_name, config_asset) {
            Tool::Known(tool_info) => {
                if let Err(e) = self.sync_single_tool(&tool_info, &pb_msg) {
                    self.sync_progress.failure(pb_msg, tool_name, format!("[error] {}", e));
                } else {
                    self.sync_progress.success(pb_msg, tool_name);
                }
            },
            Tool::Error(e) => {
                self.sync_progress.failure(pb_msg, tool_name, e.display());
            },
        }
    }

    fn sync_single_tool(&self, tool_info: &ToolInfo, pb_msg: &ProgressBar) -> Result<(), Box<dyn Error>> {
        match tool_info.asset_name.get_name_by_os() {
            None => {
                Err("Unknown asset name for the current OS".into())
            }
            Some(asset_name) => {
                let downloader = Downloader {
                    owner: &tool_info.owner,
                    repo: &tool_info.repo,
                    sync_progress: &self.sync_progress,
                    pb_msg,
                    asset_name,
                };
    
                let download_info = downloader.download(self.tmp_dir.path())?;
    
                let archive = Archive::from(
                    &download_info.archive_path,
                    self.tmp_dir.path(),
                    &tool_info.exe_name,
                    &download_info.asset_name,
                );
    
                match archive {
                    None => {
                        Err(format!("Unsupported archive type: {}", download_info.asset_name).into())
                    },
                    Some(archive) => {
                        let tool_path = archive.unpack()?;
                        copy_file(tool_path, &self.store_directory, &tool_info.exe_name)?;

                        Ok(())
                    }
                }
            }
        }
    }

}


fn copy_file(tool_path: PathBuf, store_directory: &PathBuf, exe_name: &str) -> std::io::Result<()> {
    let mut install_path = PathBuf::new();
    install_path.push(store_directory);
    install_path.push(exe_name);

    // Copy file from the downloaded unpacked archive to 'store_directory'
    fs::copy(tool_path, install_path)?;  

    Ok(())
}