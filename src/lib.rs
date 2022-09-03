mod config;
mod config_template;
mod err;
mod model;
mod sync;

use clap::Parser;
use std::path::PathBuf;

use crate::config::cli::{Cli, Command};
use crate::config::toml;
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
        Command::Generate => {
            if let Err(e) = generate_config(cli.config.clone()) {
                eprint!("{}", e);
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

fn generate_config(config_path: Option<PathBuf>) -> Result<(), std::io::Error> {
    let path = resolve_config_path(config_path);
    std::fs::write(path, config_template::CONFIG_TEMPLATE)?;
    Ok(())
}
