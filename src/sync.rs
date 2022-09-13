mod archive;
mod configure;
pub mod db;
mod download;
mod install;
mod prefetch;
mod progress;

use console::Emoji;

use crate::config::schema::Config;

use self::install::Installer;
use self::prefetch::prefetch;
use self::progress::SyncProgress;
use self::progress::ToolPair;

pub fn sync(config: Config) {
    if config.tools.is_empty() {
        no_tools_message()
    } else {
        sync_tools(config)
    }
}

fn no_tools_message() {
    eprintln!(
        r#"No tools to sync. Have you configured 'tool-sync'?

Put the following into the $HOME/.tool.toml file for the simplest configuration:

    # ensure this directory is listed in $PATH
    store_directory = "/path/to/install/directory"

    [bat]
    [exa]
    [fd]
    [ripgrep]

For more details, refer to the official documentation:

    * https://github.com/chshersh/tool-sync#tool-sync"#
    );
}

const DONE: Emoji<'_, '_> = Emoji("✨ ", "* ");
const DIRECTORY: Emoji<'_, '_> = Emoji("📁 ", "* ");

fn sync_tools(config: Config) {
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
        let is_successs = installer.install(tool_asset);
        if is_successs {
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
