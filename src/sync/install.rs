use indicatif::ProgressBar;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use tempdir::TempDir;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use crate::err;
use crate::model::asset_name::mk_exe_name;
use crate::model::tool::ToolAsset;

use super::archive::Archive;
use super::download::Downloader;
use super::progress::SyncProgress;

pub struct Installer<'a> {
    store_directory: &'a Path,
    tmp_dir: TempDir,
    sync_progress: SyncProgress,
}

impl<'a> Installer<'a> {
    /// This functions panics when it can't create a temporary directory
    /// (e.g. not enough disk space?)
    pub fn mk(store_directory: &Path, sync_progress: SyncProgress) -> Installer {
        let tmp_dir = TempDir::new("tool-sync");
        match tmp_dir {
            Err(e) => {
                err::abort_suggest_issue(&format!("Error creating temporary directory: {}", e));
            }
            Ok(tmp_dir) => Installer {
                store_directory,
                tmp_dir,
                sync_progress,
            },
        }
    }

    /// Returns `true` if the installation was successful
    pub fn install(&self, tool_asset: ToolAsset) -> bool {
        let tool_name = &tool_asset.tool_name;
        let tag = &tool_asset.tag;

        let pb_msg = self.sync_progress.create_message_bar(tool_name, tag);

        match self.sync_single_tool(&tool_asset, &pb_msg) {
            Ok(_) => {
                self.sync_progress.success(pb_msg, tool_name, tag);
                true
            }
            Err(e) => {
                self.sync_progress
                    .failure(pb_msg, tool_name, tag, format!("[error] {}", e));
                false
            }
        }
    }

    fn sync_single_tool(
        &self,
        tool_asset: &ToolAsset,
        pb_msg: &ProgressBar,
    ) -> Result<(), Box<dyn Error>> {
        let downloader = Downloader {
            asset: &tool_asset.asset,
            client: &tool_asset.client,
            sync_progress: &self.sync_progress,
            pb_msg,
        };

        let download_info = downloader.download(self.tmp_dir.path())?;

        let archive = Archive::from(
            &download_info.archive_path,
            self.tmp_dir.path(),
            &tool_asset.exe_name,
            &tool_asset.asset.name,
        );

        match archive {
            None => Err(format!("Unsupported asset type: {}", tool_asset.asset.name).into()),
            Some(archive) => match archive.unpack() {
                Err(unpack_err) => Err(unpack_err.display().into()),
                Ok(tool_path) => {
                    copy_file(tool_path, self.store_directory, &tool_asset.exe_name)?;
                    Ok(())
                }
            },
        }
    }
}

fn copy_file(tool_path: PathBuf, store_directory: &Path, exe_name: &str) -> std::io::Result<()> {
    let exe_name = mk_exe_name(exe_name);

    let mut install_path = PathBuf::new();
    install_path.push(store_directory);
    install_path.push(exe_name);

    // Copy file from the downloaded unpacked archive to 'store_directory'
    fs::copy(tool_path, &install_path)?;

    set_executable_permissions(&install_path);

    Ok(())
}

#[cfg(target_family = "windows")]
fn set_executable_permissions(_exe_path: &PathBuf) {}

#[cfg(target_family = "unix")]
fn set_executable_permissions(exe_path: &PathBuf) {
    fs::set_permissions(exe_path, fs::Permissions::from_mode(0o755)).unwrap();
}
