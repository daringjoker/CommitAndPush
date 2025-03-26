use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, Clone)]
pub enum SubCommand {
    Review,
    Pr,
    Critique,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub subcmd: SubCommand,
}

impl Arguments {
    pub fn parse_all() -> Arguments {
        Arguments::parse()
    }
}
