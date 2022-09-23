use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub struct Cli {
    /// Sets a path to a configuration file (default: $HOME/.tool.toml)
    #[clap(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Sync all tools specified in configuration file or the only one specified in the command line
    Sync { tool: Option<String> },

    /// Generate a default .tool.toml file and prints it to std out
    DefaultConfig,

    /// Install a tool if it is hardcoded into internal database
    Install { name: String },
}
