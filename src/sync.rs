mod archive;
mod configure;
pub mod db;
mod download;
mod install;
mod prefetch;
mod progress;

use console::Emoji;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::config::schema::{Config, ConfigAsset};
use crate::config::toml;

use self::install::Installer;
use self::prefetch::prefetch;
use self::progress::SyncProgress;
use self::progress::ToolPair;

pub fn sync_from_path(config_path: PathBuf, tool: Option<String>) {
    toml::with_parsed_file(config_path.clone(), |config| {
        sync_from_config(config, config_path, tool)
    });
}

pub fn sync_from_config(mut config: Config, config_path: PathBuf, tool: Option<String>) {
    if config.tools.is_empty() {
        no_tools_message();
        return;
    }

    match tool {
        Some(tool) => match config.tools.remove(&tool) {
            Some(asset) => sync_single_tool(config, tool, asset),
            None => tool_not_in_config_message(&tool, &config_path),
        },
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

fn tool_not_in_config_message(tool: &str, path: &Path) {
    eprintln!(
        r#"The '{}' tool is not listed in the configuration file: {}

Add the tool to the configuration file or use the 'tool install' command for 
installing one of the tools natively supported by 'tool-sync'."#,
        tool,
        path.display(),
    );
}

const DONE: Emoji<'_, '_> = Emoji("✨ ", "* ");
const DIRECTORY: Emoji<'_, '_> = Emoji("📁 ", "* ");

pub fn sync_single_tool(mut config: Config, name: String, asset: ConfigAsset) {
    config.tools = BTreeMap::from([(name, asset)]);
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

    summary_message(installed_tools, store_directory);
}

fn summary_message(installed_tools: u64, store_directory: PathBuf) {
    eprintln!(
        "{} Successfully installed {} {}!",
        DONE,
        installed_tools,
        if installed_tools == 1 {
            "tool"
        } else {
            "tools"
        }
    );
    eprintln!(
        "{} Installation directory: {}",
        DIRECTORY,
        store_directory.display()
    );
}
