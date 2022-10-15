/// This file only holds the template that is used to generate a default .tool.toml.
use clap_complete::Shell;

use crate::sync::db;

pub fn generate_default_config() {
    println!("{}", config_template());
}

fn config_template() -> String {
    let tools = db::fmt_tool_names_info(|name| format!("# [{name}]"));

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

// This function can break when clap_complete adds support for a new shell type
pub fn rename_completion_suggestion(shell: &Shell, bin_name: &str) -> Result<(), RenameError> {
    let completion_str: String = match shell {
        Shell::Zsh => format!(r##"Generate a `_{bin_name}` completion script and put it somewhere in your `$fpath`:
`{bin_name} completion zsh --rename {bin_name} > /usr/local/share/zsh/site-functions/_{bin_name}`

Ensure that the following is present in your `~/.zshrc`:

`autoload -U compinit`

`compinit -i`"##),
        Shell::Bash => format!(r##"First, ensure that you install `bash-completion` using your package manager.

After, add this to your `~/.bash_profile`:

`eval "$({bin_name} completion bash --rename {bin_name})"`"##),
        Shell::Fish => format!(r##"Generate a `tool.fish` completion script:

`{bin_name} completion fish --rename {bin_name} > ~/.config/fish/completions/{bin_name}.fish`"##),
        Shell::Elvish => r##"This suggestion is missing, if you use this and know how to implement this please file an issue over at https://github.com/chshersh/tool-sync/issues"##.into(),
        Shell::PowerShell => format!(r##"Open your profile script with:

`mkdir -Path (Split-Path -Parent $profile) -ErrorAction SilentlyContinue`
`notepad $profile`

Add the line and save the file:

`Invoke-Expression -Command $({bin_name} completion powershell --rename {bin_name} | Out-String)`"##),
        _ => return Err(RenameError::NewShellFound(shell.to_owned())),
    };

    eprintln!(
        "\n\n############################\n{}\n############################",
        completion_str
    );

    Ok(())
}

pub enum RenameError {
    NewShellFound(Shell),
}
