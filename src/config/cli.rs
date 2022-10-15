use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub struct Cli {
    /// Set a path to a configuration file (default: $HOME/.tool.toml)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    ///Use with a custom proxy like this: `tool --proxy=http://127.0.0.1:8080 sync`
    #[arg(short, long, value_name = "uri")]
    pub proxy: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
