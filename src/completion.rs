//! This file contains all logic revolving the generation of the shell completion script

use clap_complete::Shell;

// This function can break when clap_complete adds support for a new shell type
pub fn rename_completion_suggestion(shell: &Shell, bin_name: &str) -> Result<(), RenameError> {
    let completion_str: String = match shell {
        Shell::Zsh => zsh_completion_help(bin_name),
        Shell::Bash => bash_completion_help(bin_name),
        Shell::Fish => fish_completion_help(bin_name),
        Shell::Elvish => elvish_completion_help(bin_name),
        Shell::PowerShell => powershell_completion_help(bin_name),
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

impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            RenameError::NewShellFound(shell) => write!(f, "[Rename error]: {}", shell),
        }
    }
}

//##################//
// Helper functions //
//##################//

fn zsh_completion_help(bin_name: &str) -> String {
    format!(
        r##"Generate a `_{bin_name}` completion script and put it somewhere in your `$fpath`:
`{bin_name} completion zsh --rename {bin_name} > /usr/local/share/zsh/site-functions/_{bin_name}`

Ensure that the following is present in your `~/.zshrc`:

`autoload -U compinit`

`compinit -i`"##
    )
}

fn bash_completion_help(bin_name: &str) -> String {
    format!(
        r##"First, ensure that you install `bash-completion` using your package manager.

After, add this to your `~/.bash_profile`:

`eval "$({bin_name} completion bash --rename {bin_name})"`"##
    )
}

fn fish_completion_help(bin_name: &str) -> String {
    format!(
        r##"Generate a `tool.fish` completion script:

`{bin_name} completion fish --rename {bin_name} > ~/.config/fish/completions/{bin_name}.fish`"##
    )
}

fn elvish_completion_help(_bin_name: &str) -> String {
    r##"This suggestion is missing, if you use this and know how to implement this please file an issue over at https://github.com/chshersh/tool-sync/issues"##.into()
}

fn powershell_completion_help(bin_name: &str) -> String {
    format!(
        r##"Open your profile script with:

`mkdir -Path (Split-Path -Parent $profile) -ErrorAction SilentlyContinue`
`notepad $profile`

Add the line and save the file:

`Invoke-Expression -Command $({bin_name} completion powershell --rename {bin_name} | Out-String)`"##
    )
}
