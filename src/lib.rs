mod config;
mod infra;
mod install;
mod model;
mod sync;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use std::path::PathBuf;

use crate::config::cli::{Cli, Command};
use crate::infra::err;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();

    // TODO: this is redundant for the `default-config` command
    // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(cli.config);

    match cli.command {
        Command::Completion { shell } => {
            generate_completion(shell);
        },
        Command::DefaultConfig { path } => match path {
            true => print_default_path(),
            false => config::template::generate_default_config(),
        },
        Command::Sync { tool } => sync::sync_from_path(config_path, tool, cli.proxy),
        Command::Install { name } => install::install(config_path, name, cli.proxy),
    }
}

fn generate_completion(shell: clap_complete::Shell) {
    let mut cmd: clap::Command = crate::config::cli::Cli::command();
    let cmd_name: String = cmd.get_name().into();

    generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());

}

fn resolve_config_path(config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(path) => path,
        None => get_default_config_path(),
    }
}

fn get_default_config_path() -> PathBuf {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut path = PathBuf::new();
            path.push(home_path);
            path.push(DEFAULT_CONFIG_PATH);
            path
        }
        None => {
            err::abort_suggest_issue("Unable to find $HOME directory");
        }
    }
}

fn print_default_path() {
    println!("{}", get_default_config_path().display());
}
