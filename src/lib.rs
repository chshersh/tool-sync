mod config;
mod err;
mod infra;
mod model;
mod sync;

use std::collections::BTreeMap;
use std::path::PathBuf;

use clap::Parser;

use crate::config::cli::{Cli, Command};
use crate::config::schema::ConfigAsset;
use crate::config::template;
use crate::config::toml;
use crate::sync::db::lookup_tool;
use crate::sync::sync;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();
    let config_path = resolve_config_path(cli.config.clone());

    match cli.command {
        Command::Sync => match toml::parse_file(&config_path) {
            Err(e) => {
                err::abort_with(&format!(
                    "Error parsing configuration at path {}: {}",
                    config_path.display(),
                    e.display()
                ));
            }
            Ok(tool) => {
                sync(tool);
            }
        },
        Command::DefaultConfig => generate_config(),
        Command::Install { name } => {
            if let Some(tool_info) = lookup_tool(&name) {
                if let Ok(mut tool) = toml::parse_file(&config_path) {
                    let tool_btree: BTreeMap<String, ConfigAsset> =
                        BTreeMap::from([(name, tool_info.into())]);
                    tool.tools = tool_btree;
                    sync(tool);
                }
            }
        }
    }
}

fn resolve_config_path(config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(path) => path,
        None => match dirs::home_dir() {
            Some(home_path) => {
                let mut path = PathBuf::new();
                path.push(home_path);
                path.push(DEFAULT_CONFIG_PATH);
                path
            }
            None => {
                err::abort_suggest_issue("Unable to find $HOME directory");
            }
        },
    }
}

fn generate_config() {
    println!("{}", template::config_template());
}
