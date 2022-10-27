

// This deny block can be circumvent by adding the attribute #[allow(clippy::print_stdout)] to your
// function or method. This block exists here to prevent stray debug prints from making it into the
// main branch.
//
// See
//  https://github.com/chshersh/tool-sync/issues/148
//  https://github.com/chshersh/tool-sync/issues/156
#![deny(clippy::print_stdout)]

#[deny(clippy::print_stdout)]
mod completion;
#[deny(clippy::print_stdout)]
mod config;
#[deny(clippy::print_stdout)]
mod infra;
#[deny(clippy::print_stdout)]
mod install;
#[deny(clippy::print_stdout)]
mod model;
#[deny(clippy::print_stdout)]
mod sync;

use clap::{CommandFactory, Parser};
use clap_complete::generate;

use std::path::PathBuf;

use crate::completion::rename_completion_suggestion;
use crate::config::cli::{Cli, Command};
use crate::infra::err;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();

    // TODO: this is redundant for the `default-config` command
    // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(cli.config);
    println!("Should not compile");

    match cli.command {
        Command::Completion { shell, rename } => {
            generate_completion(shell, rename);
        }
        Command::DefaultConfig { path } => match path {
            true => print_default_path(),
            false => config::template::generate_default_config(),
        },
        Command::Sync { tool } => sync::sync_from_path(config_path, tool, cli.proxy),
        Command::Install { name } => install::install(config_path, name, cli.proxy),
    }
}

fn generate_completion(shell: clap_complete::Shell, rename: Option<String>) {
    let mut cmd: clap::Command = Cli::command();
    match rename {
        Some(cmd_name) => {
            generate(shell, &mut cmd, &cmd_name, &mut std::io::stdout());
            rename_completion_suggestion(&shell, &cmd_name)
                .unwrap_or_else(|e| err::abort_suggest_issue(e));
        }
        None => {
            let cmd_name: String = cmd.get_name().into();
            generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
        }
    };
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

#[allow(clippy::print_stdout)]
fn print_default_path() {
    println!("{}", get_default_config_path().display());
}
