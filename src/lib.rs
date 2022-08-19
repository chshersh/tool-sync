mod config;
mod model;
mod sync;

use std::path::PathBuf;
use std::process;
use clap::Parser;
use dirs;

use crate::config::cli::{Cli, Command};
use crate::config::toml::parse_config;
use crate::sync::sync;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();
    let config_path = resolve_config_path(cli.config);

    match parse_config(&config_path) {
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        },
        Ok(tool) => match cli.command {
           Command::Sync => sync(tool),
        },
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
                eprintln!("Unable to find $HOME directory");
                process::exit(1);
            }
        }
    }
}