use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use tempdir::TempDir;

use crate::model::release::Release;
use crate::config::schema::ConfigAsset;
use crate::model::tool::{Tool, ToolInfo};
use crate::sync::archive::Archive;
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
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest",
                              owner = tool_info.owner,
                              repo = tool_info.repo);
    println!("{}", request_url);
    let release: Release =
        ureq::get(&request_url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "chshersh/tool-sync-1.0")
        .call()?
        .into_json()?;

    println!("{:#?}", release);

    match tool_info.asset_name.get_name_by_os() {
        None => eprintln!("Unknown asset name for the current OS"),
        Some(asset_name) => {
            let asset = release
                    .assets
                    .iter()
                    .find(|&asset| asset.name.contains(asset_name));
            
            match asset {
                None => eprintln!("No asset matching name: {}", asset_name),
                Some(asset) => {
                    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/assets/{asset_id}",
                                              owner = tool_info.owner,
                                              repo = tool_info.repo,
                                              asset_id = asset.id);
        
                    println!("{}", request_url);
        
                    let mut stream = ureq::get(&request_url)
                        .set("Accept", "application/octet-stream")
                        .set("User-Agent", "chshersh/tool-sync-1.0")
                        .call()?
                        .into_reader();
        
                    let download_path = tmp_dir.path().join(&asset.name);
                    let mut destination = File::create(&download_path)?;
                    let mut buffer = [0; 4096];
                    while let Ok(bytes_read) = stream.read(&mut buffer) {
                        if bytes_read == 0 {
                            break;
                        }
        
                        destination.write(&buffer[..bytes_read])?;
                    }

                    let archive = Archive::from(
                        &download_path,
                        tmp_dir.path(),
                        &tool_info.exe_name,
                        &asset.name,
                    );

                    match archive {
                        None => {
                            eprintln!("Unsupported archive type: {}", asset.name);
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