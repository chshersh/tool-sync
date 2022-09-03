mod archive;
mod configure;
mod db;
mod download;
mod install;
mod progress;

use crate::config::schema::Config;
use crate::sync::install::Installer;
use crate::sync::progress::SyncProgress;

pub fn sync(config: Config) {
    if config.tools.is_empty() {
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
    } else {
        let store_directory = config.ensure_store_directory();

        let tools: Vec<String> = config.tools.keys().cloned().collect();
        let tags: Vec<String> = config
            .tools
            .values()
            .map(|config_asset| { config_asset.tag.clone().unwrap_or_else(|| "latest".into()) })
            .collect();
        let sync_progress = SyncProgress::new(tools, tags);
        let installer = Installer::mk(store_directory, sync_progress);

        for (tool_name, config_asset) in config.tools.iter() {
            installer.install(tool_name, config_asset);
        }
    }
}
