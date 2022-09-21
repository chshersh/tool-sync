mod archive;
mod configure;
pub mod db;
mod download;
mod install;
mod prefetch;
mod progress;

use console::Emoji;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::config::schema::Config;
use crate::config::toml;

use self::install::Installer;
use self::prefetch::prefetch;
use self::progress::SyncProgress;
use self::progress::ToolPair;

pub fn sync_from_path(config_path: PathBuf, tool: Option<String>) {
    toml::with_parsed_file(config_path, |config| sync_from_config(config, tool));
}

pub fn sync_from_config(config: Config, tool: Option<String>) {
    if config.tools.is_empty() {
        no_tools_message();
        return;
    }

    match tool {
        Some(tool) => {
            if config.tools.contains_key(&tool) {
                sync_tool_no_check(config, tool);
            } else {
                tool_not_in_config_message(&tool);
            }
        }
        None => sync_from_config_no_check(config),
    }
}

fn no_tools_message() {
    eprintln!(
        r#"No tools to sync. Have you configured 'tool-sync'?

Put the following into the $HOME/.tool.toml file for the simplest configuration:

### START ###

# ensure this directory is listed in $PATH
store_directory = "/path/to/install/directory"

[bat]
[exa]
[fd]
[ripgrep]

### END ###

For more details, refer to the official documentation:

    * https://github.com/chshersh/tool-sync#tool-sync"#
    );
}

fn tool_not_in_config_message(tool: &str) {
    eprintln!(
        r#"Tool: {} is not specified in the configuration file.

Consider adding it to the configuration file or use tool install command
to install it if it's a known tool.
        "#,
        tool
    );
}

const DONE: Emoji<'_, '_> = Emoji("✨ ", "* ");
const DIRECTORY: Emoji<'_, '_> = Emoji("📁 ", "* ");

fn sync_tool_no_check(mut config: Config, tool: String) {
    let tool_config_asset = (*config.tools.get(&tool).unwrap()).clone();

    let mut tool_btree = BTreeMap::new();
    tool_btree.insert(tool.clone(), tool_config_asset);

    config.tools = tool_btree;

    sync_from_config_no_check(config);
}

/// Like `sync_from_config` but expects non-empty list of tools
pub fn sync_from_config_no_check(config: Config) {
    let store_directory = config.ensure_store_directory();
    let tool_assets = prefetch(config.tools);

    let tool_pairs = tool_assets
        .iter()
        .map(|ta| ToolPair {
            name: &ta.tool_name,
            tag: &ta.tag,
        })
        .collect();

    let sync_progress = SyncProgress::new(tool_pairs);
    let installer = Installer::mk(store_directory.as_path(), sync_progress);

    let mut installed_tools: u64 = 0;

    for tool_asset in tool_assets {
        let is_success = installer.install(tool_asset);
        if is_success {
            installed_tools += 1
        }
    }

    eprintln!("{} Successfully installed {} tools!", DONE, installed_tools);
    eprintln!(
        "{} Installation directory: {}",
        DIRECTORY,
        store_directory.display()
    );
}
