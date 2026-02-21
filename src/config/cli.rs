use std::path::PathBuf;

use facet_args as args;

#[derive(facet::Facet, Default)]
// #[command(author="Dmitrii Kovanikov <kovanikov@gmail.com>", version, about="A CLI tool to manage other CLI tools", long_about = None)]
pub(crate) struct Args {
    #[facet(args::named, args::short = 'c')]
    pub config: Option<PathBuf>,
    #[facet(args::named, args::short = 'p')]
    pub proxy: Option<String>,

    #[facet(args::positional)]
    pub tools: Vec<String>,
}
