use std::process;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use ureq;

use crate::tool::{Tool, ToolDetails};

#[derive(Deserialize, Debug)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    id: u32,
    name: String,
    size: u32,
}

pub fn sync(tool: Tool) {
    let has_store_directory = tool.store_directory.as_path().is_dir();

    if !has_store_directory {
        eprintln!("Specified directory for storing tools doesn't exist: {}", tool.store_directory.display());
        process::exit(1);
    }

    for (tool_name, tool_details) in tool.tools.iter() {
        if tool_name == "ripgrep" {
            println!("Downloading rg!");
            if let Err(e) = sync_single_tool(tool_name, tool_details) {
                eprintln!("Error syncing a tool: {e}");
            }
        } else {
            println!("Skipping: {}", tool_name);
        }
    }
}

fn sync_single_tool(tool_name: &str, tool_details: &ToolDetails) -> Result<(), Box<dyn Error>> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest",
                              owner = "BurntSushi",
                              repo = "ripgrep");
    println!("{}", request_url);
    let release: Release =
        ureq::get(&request_url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "chshersh/tool-sync-1.0")
        .call()?
        .into_json()?;

    println!("{:#?}", release);

    for asset in release.assets {
        if asset.name == "ripgrep-13.0.0-x86_64-unknown-linux-musl.tar.gz" {
            let request_url = format!("https://api.github.com/repos/{owner}/{repo}/releases/assets/{asset_id}",
                                      owner = "BurntSushi",
                                      repo = "ripgrep",
                                      asset_id = asset.id);

            println!("{}", request_url);

            let mut stream = ureq::get(&request_url)
                .set("Accept", "application/octet-stream")
                .set("User-Agent", "chshersh/tool-sync-1.0")
                .call()?
                .into_reader();

            let mut destination = File::create(asset.name)?;
            let mut buffer = [0; 4096];
            while let Ok(bytes_read) = stream.read(&mut buffer) {
                if bytes_read == 0 {
                    break;
                }

                destination.write(&buffer[..bytes_read])?;
            }
        }
    }

    Ok(())
}