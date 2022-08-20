use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use tempdir::TempDir;

use crate::config::schema::ConfigAsset;
use crate::model::tool::{Tool, ToolInfo};
use crate::sync::archive::Archive;
use crate::sync::download::Downloader;
use crate::sync::configure::configure_tool;

pub struct Installer {
    store_directory: PathBuf,
    tmp_dir: TempDir,
}

impl Installer {
    pub fn from(store_directory: PathBuf) -> Installer {
        let tmp_dir = TempDir::new("tool-sync");
        match tmp_dir {
            Err(e) => {
                eprintln!("Error creating temporary directory: {}", e);
                process::exit(1);
            },
            Ok(tmp_dir) => Installer
                { store_directory
                , tmp_dir
                }
        }
    }

    pub fn install(&self, tool_name: &str, config_asset: &ConfigAsset) {
        match configure_tool(tool_name, config_asset) {
            Tool::Known(tool_info) => {
                eprintln!("Installing: {}", tool_name);
                if let Err(e) = sync_single_tool(&self.store_directory, &self.tmp_dir, tool_name, &tool_info) {
                   eprintln!("Error syncing a tool: {e}");
                }
            },
            Tool::Error(e) => {
                // TODO: Better error reporting here
                eprintln!("Unknown tool: {}", tool_name);
            },
        }

    }
}

fn sync_single_tool(store_directory: &PathBuf, tmp_dir: &TempDir, tool_name: &str, tool_info: &ToolInfo) -> Result<(), Box<dyn Error>> {
    match tool_info.asset_name.get_name_by_os() {
        None => eprintln!("Unknown asset name for the current OS"),
        Some(asset_name) => {
            let downloader = Downloader {
                owner: &tool_info.owner,
                repo: &tool_info.repo,
                asset_name,
            };

            let download_info = downloader.download(tmp_dir.path())?;

            let archive = Archive::from(
                &download_info.archive_path,
                tmp_dir.path(),
                &tool_info.exe_name,
                &download_info.asset_name,
            );

            match archive {
                None => {
                    eprintln!("Unsupported archive type: {}", download_info.asset_name);
                    process::exit(1);
                },
                Some(archive) => {
                    println!("Starting to unpack...");
                    let tool_path = archive.unpack()?;
                    copy_file(tool_path, store_directory, &tool_info.exe_name)?;
                }
            }
        }
    }

    Ok(())
}

fn copy_file(tool_path: PathBuf, store_directory: &PathBuf, exe_name: &str) -> std::io::Result<()> {
    let mut install_path = PathBuf::new();
    install_path.push(store_directory);
    install_path.push(exe_name);

    eprintln!("Copy from: {}", tool_path.display());
    eprintln!("Copy to:   {}", install_path.display());
    
    // Copy file from the downloaded unpacked archive to 'store_directory'
    fs::copy(tool_path, install_path)?;  

    Ok(())
}