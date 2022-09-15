mod config;
mod err;
mod infra;
mod model;
mod sync;

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::PathBuf;

use clap::Parser;

use crate::config::cli::{Cli, Command};
use crate::config::schema::{Config, ConfigAsset};
use crate::config::template;
use crate::config::toml;
use crate::sync::db::{build_db, lookup_tool};
use crate::sync::sync;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();
    let config_path = resolve_config_path(cli.config.clone());

    match cli.command {
        Command::DefaultConfig => generate_config(),
        Command::Sync => match toml::parse_file(&config_path) {
            Err(e) => {
                err::abort_with(&format!(
                    "Error parsing configuration at path {}: {}",
                    config_path.display(),
                    e.display()
                ));
            }
            Ok(config) => {
                sync(config);
            }
        },
        Command::Install { name } => {
            with_parsed_file(&config_path, |mut config| {
                if let Some(tool_info) = lookup_tool(&name) {
                    let tool_btree: BTreeMap<String, ConfigAsset> =
                        BTreeMap::from([(name, tool_info.into())]);
                    config.tools = tool_btree;
                    sync(config);
                } else {
                    let mut exit_message: String =
                        format!("Unknown tool: {}\nSupported tools:\n", name);
                    for tool in build_db().keys().cloned().collect::<Vec<String>>() {
                        if let Err(e) = writeln!(exit_message, "\t* {}", tool) {
                            err::abort_suggest_issue(&format!("{}", e));
                        };
                    }
                    err::abort_with(&exit_message);
                };
            });
        }
    }
}

fn with_parsed_file<F: FnOnce(Config)>(config_path: &PathBuf, on_success: F) {
    match toml::parse_file(config_path) {
        Ok(config) => {
            on_success(config);
        }
        Err(e) => {
            err::abort_with(&format!(
                "Error parsing configuration at path {}: {}",
                config_path.display(),
                e.display()
            ));
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
