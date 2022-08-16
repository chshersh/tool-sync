mod cli;

use std::path::PathBuf;
use std::process;
use clap::Parser;
use dirs;

use cli::Cli;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let config_path = resolve_config_path(cli.config);

    println!("Config path: {}", config_path.display());
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