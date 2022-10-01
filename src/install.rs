use std::path::PathBuf;

use crate::config::schema::Config;
use crate::config::toml;
use crate::infra::err;
use crate::sync;
use crate::sync::db::{build_db, fmt_tool_names_info, lookup_tool};

/// Install a single tool
pub fn install(config_path: PathBuf, name: String) {
    toml::with_parsed_file(config_path, |config| install_tool(config, name))
}

/// Find if the tool is already mentioned in the config
fn install_tool(config: Config, name: String) {
    if let Some(tool_info) = lookup_tool(&name) {
        sync::sync_single_tool(config, name, tool_info.into());
    } else {
        let max_name_length: usize = build_db().keys().map(|a| a.len()).max().unwrap() + 1;

        let tools = fmt_tool_names_info(|(name, info)| {
            format!(
                "    * {name} {delim:>padding$} https://github.com/{owner}/{repo}",
                delim = "#",
                padding = max_name_length - name.len(),
                owner = info.owner,
                repo = info.repo,
            )
        });

        let exit_message = format!(
            r#"Unknown tool: '{name}'
Supported tools:
{tools}"#
        );

        err::abort(exit_message);
    }
}
