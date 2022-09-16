mod config;
mod infra;
mod install;
mod model;
mod sync;

use clap::Parser;
use std::path::PathBuf;

use crate::config::cli::{Cli, Command};
use crate::infra::err;

pub fn run() {
    let cli = Cli::parse();

    // TODO: this is redundant for the `default-config` command
    // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(cli.config);

    match cli.command {
        Command::DefaultConfig => config::template::generate_default_config(),
        Command::Sync => sync::sync_from_path(config_path),
        Command::Install { name } => install::install(config_path, name),
    }
}

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

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
