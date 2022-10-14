use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub struct Cli {
    /// Sets a path to a configuration file (default: $HOME/.tool.toml)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[arg(short, long, value_name = "uri")]
    pub proxy: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate shell completion scripts for GitHub CLI commands.
    ///
    /// You will need to set up completions manually, follow the instructions below. The exact
    /// config file locations might vary based on your system. Make sure to restart your
    /// shell before testing whether completions are working.
    ///
    /// ### bash
    ///
    /// First, ensure that you install `bash-completion` using your package manager.
    ///
    /// After, add this to your `~/.bash_profile`:
    ///
    /// `eval "$(tool completion bash)"`
    ///
    /// ### zsh
    ///
    /// Generate a `_tool` completion script and put it somewhere in your `$fpath`:
    ///
    /// `tool completion zsh > /usr/local/share/zsh/site-functions/_tool`
    ///
    /// Ensure that the following is present in your `~/.zshrc`:
    ///
    /// `autoload -U compinit`
    /// `compinit -i`
    ///
    /// Zsh version 5.7 or later is recommended.
    ///
    /// ### fish
    ///
    /// Generate a `tool.fish` completion script:
    ///
    /// `tool completion fish > ~/.config/fish/completions/tool.fish`
    ///
    /// ### PowerShell
    ///
    /// Open your profile script with:
    ///
    /// `mkdir -Path (Split-Path -Parent $profile) -ErrorAction SilentlyContinue`
    /// `notepad $profile`
    ///
    /// Add the line and save the file:
    ///
    /// `Invoke-Expression -Command $(tool completion powershell | Out-String)`
    Completion { shell: clap_complete::Shell },

    /// Sync all tools specified in configuration file or the only one specified in the command line
    Sync { tool: Option<String> },

    /// Print a default .tool.toml configuration to std out
    DefaultConfig {
        /// Print the default config file location instead
        #[arg(long)]
        path: bool,
    },

    /// Install a tool if it is hardcoded into internal database
    Install { name: String },
}
