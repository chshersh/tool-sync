mod config;
mod infra;
mod model;
mod sync;

use std::path::PathBuf;

use crate::config::cli::Args;
use crate::infra::err;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let args: Args = facet_args::from_std_args().expect("parsable args");

    // TODO: this is redundant for the `default-config` command
    // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(args.config);
    for tool in args.tools {
        sync::sync_from_path(config_path.clone(), Some(&tool), args.proxy.clone())
    }
}

fn resolve_config_path(config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(path) => path,
        None => get_default_config_path(),
    }
}
fn get_default_config_path() -> PathBuf {
    match std::env::home_dir() {
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
