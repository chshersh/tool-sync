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
        eprintln!("Configuration doesn't list any tools");
    } else {
        config.ensure_store_directory();

        let tools: Vec<String> = config.tools.keys().cloned().collect();
        let sync_progress = SyncProgress::new(tools);
        let installer = Installer::mk(config.store_directory, sync_progress);

        for (tool_name, config_asset) in config.tools.iter() {
            installer.install(tool_name, config_asset);
        }
    }
}