mod config;
mod infra;
mod install;
mod model;
mod sync;

use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};

use std::path::PathBuf;

use crate::config::cli::{Cli, Command};
use crate::infra::err;

const DEFAULT_CONFIG_PATH: &str = ".tool.toml";

pub fn run() {
    let cli = Cli::parse();

    // TODO: this is redundant for the `default-config` command
    // See: https://github.com/chshersh/tool-sync/issues/75
    let config_path = resolve_config_path(cli.config);

    match cli.command {
        Command::Completion { shell, rename } => {
            generate_completion(shell, rename);
        }
        Command::DefaultConfig { path } => match path {
            true => print_default_path(),
            false => config::template::generate_default_config(),
        },
        Command::Sync { tool } => sync::sync_from_path(config_path, tool, cli.proxy),
        Command::Install { name } => install::install(config_path, name, cli.proxy),
    }
}

fn generate_completion(shell: clap_complete::Shell, rename: Option<String>) {
    let mut cmd: clap::Command = Cli::command();
    match rename {
        Some(cmd_name) => {
            generate(shell, &mut cmd, &cmd_name, &mut std::io::stdout());
            rename_completion_suggestion(&shell, &cmd_name)
                .unwrap_or_else(|e| err::abort_suggest_issue(e));
        }
        None => {
            let cmd_name: String = cmd.get_name().into();
            generate(shell, &mut cmd, cmd_name, &mut std::io::stdout());
        }
    };
}

// This function can break when clap_complete adds support for a new shell type
fn rename_completion_suggestion(shell: &Shell, bin_name: &str) -> Result<(), RenameError> {
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

enum RenameError {
    NewShellFound(Shell),
}

impl std::fmt::Display for RenameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            RenameError::NewShellFound(shell) => write!(f, "[Rename error]: {}", shell),
        }
    }
}

fn resolve_config_path(config_path: Option<PathBuf>) -> PathBuf {
    match config_path {
        Some(path) => path,
        None => get_default_config_path(),
    }
}

fn get_default_config_path() -> PathBuf {
    match dirs::home_dir() {
        Some(home_path) => {
            let mut path = PathBuf::new();
            path.push(home_path);
            path.push(DEFAULT_CONFIG_PATH);
            path
        }
        None => {
            err::abort_suggest_issue("Unable to find $HOME directory");
        }
    }
}

fn print_default_path() {
    println!("{}", get_default_config_path().display());
}
