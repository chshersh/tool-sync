mod cli;
mod file_config;

use std::path::PathBuf;
use std::process;
use clap::Parser;
use dirs;

use crate::cli::Cli;
use crate::file_config::parse_config;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let config_path = resolve_config_path(cli.config);

    println!("Config path: {}", config_path.display());

    if let Err(e) = parse_config(config_path) {
        eprintln!("Application error: {e}");
        process::exit(1);
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
                eprintln!("Unable to find $HOME directory!");
                process::exit(1);
            }
        }
    }
}