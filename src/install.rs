use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::config::schema::{Config, ConfigAsset};
use crate::config::toml;
use crate::infra::err;
use crate::sync;
use crate::sync::db::{fmt_tool_names, lookup_tool};

/// Install a single tool
pub fn install(config_path: PathBuf, name: String) {
    toml::with_parsed_file(config_path, |config| install_tool(config, name))
}

/// Find if the tool is already mentioned in the config
fn install_tool(mut config: Config, name: String) {
    if let Some(tool_info) = lookup_tool(&name) {
        let tool_btree: BTreeMap<String, ConfigAsset> = BTreeMap::from([(name, tool_info.into())]);
        config.tools = tool_btree;
        sync::sync_from_config_no_check(config);
    } else {
        let tools = fmt_tool_names(|name| format!("    * {name}"));

        let exit_message = format!(
            r#"Unknown tool: '{name}'
Supported tools:
{tools}"#
        );

        err::abort(&exit_message);
    }
}
