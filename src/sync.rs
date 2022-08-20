mod configure;
mod db;
mod install;

use crate::config::schema::Config;
use crate::sync::install::Installer;

pub fn sync(config: Config) {
    config.ensure_store_directory();
    let installer = Installer::from(config.store_directory);

    for (tool_name, config_asset) in config.tools.iter() {
        installer.install(tool_name, config_asset);
    }
}