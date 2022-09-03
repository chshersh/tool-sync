use indicatif::ProgressBar;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use tempdir::TempDir;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

use crate::config::schema::ConfigAsset;
use crate::err;
use crate::model::asset_name::mk_exe_name;
use crate::model::tool::{Tool, ToolInfo};

use super::archive::Archive;
use super::configure::configure_tool;
use super::download::Downloader;
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
                err::abort_suggest_issue(&format!("Error creating temporary directory: {}", e));
            }
            Ok(tmp_dir) => Installer {
                store_directory,
                tmp_dir,
                sync_progress,
            },
        }
    }

    pub fn install(&self, tool_name: &str, config_asset: &ConfigAsset) {
        let pb_msg = self.sync_progress.create_message_bar(tool_name);

        match configure_tool(tool_name, config_asset) {
            Tool::Known(tool_info) => match self.sync_single_tool(&tool_info, &pb_msg) {
                Ok(tag_name) => {
                    self.sync_progress.success(pb_msg, tool_name, &tag_name);
                }
                Err(e) => {
                    self.sync_progress
                        .failure(pb_msg, tool_name, format!("[error] {}", e));
                }
            },
            Tool::Error(e) => {
                self.sync_progress.failure(pb_msg, tool_name, e.display());
            }
        }
    }

    fn sync_single_tool(
        &self,
        tool_info: &ToolInfo,
        pb_msg: &ProgressBar,
    ) -> Result<String, Box<dyn Error>> {
        match tool_info.asset_name.get_name_by_os() {
            None => Err(
                "Don't know the asset name for this OS: specify it explicitly in the config".into(),
            ),
            Some(asset_name) => {
                let downloader = Downloader {
                    owner: &tool_info.owner,
                    repo: &tool_info.repo,
                    version: &tool_info.tag.to_str_version(),
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
                        Err(format!("Unsupported asset type: {}", download_info.asset_name).into())
                    }
                    Some(archive) => match archive.unpack() {
                        Err(unpack_err) => Err(unpack_err.display().into()),
                        Ok(tool_path) => {
                            copy_file(tool_path, &self.store_directory, &tool_info.exe_name)?;
                            Ok(download_info.tag_name)
                        }
                    },
                }
            }
        }
    }
}

fn copy_file(tool_path: PathBuf, store_directory: &PathBuf, exe_name: &str) -> std::io::Result<()> {
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
