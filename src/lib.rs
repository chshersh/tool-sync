mod cli;

use clap::Parser;

use cli::Cli;

pub fn run() {
    let args = Cli::parse();
    println!("{:?}", args);
}