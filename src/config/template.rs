/// This file only holds the template that is used to generate a default .tool.toml.
use crate::sync::db;

pub fn generate_default_config() {
    println!("{}", config_template());
}

fn config_template() -> String {
    let tools = db::fmt_tool_names(|name| format!("# [{name}]"));

    format!(
        r###"# This configuration is automatically generated by tool-sync {version}
# https://github.com/chshersh/tool-sync
#######################################
#
# Installation directory for all the tools:
# store_directory = "$HOME/.local/bin"
#
# tool-sync provides native support for some of the tools without the need to
# configure them. Uncomment all the tools you want to install with a single
# 'tool sync' command:
#
{tools}
#
# You can configure the installation of any tool by specifying corresponding options:
#
# [ripgrep]  # Name of the tool (new or one of the hardcoded to override default settings)
#     owner     = "BurntSushi"  # GitHub repository owner
#     repo      = "ripgrep"     # GitHub repository name
#     exe_name  = "rg"          # Executable name inside the asset

#     Uncomment to download a specific version or tag.
#     Without this tag latest will be used
#     tag       = "13.0.0"

#     Asset name to download on linux OSes
#     asset_name.linux = "x86_64-unknown-linux-musl"

#     Uncomment if you want to install on macOS as well
#     asset_name.macos = "apple-darwin"

#     Uncomment if you want to install on Windows as well
#     asset_name.windows = "x86_64-pc-windows-msvc""###,
        version = env!("CARGO_PKG_VERSION"),
    )
}
