use clap::{Parser, Subcommand};

use crate::config::LocalConfig;

#[derive(Debug, Subcommand, Clone)]
pub enum SubCommand {
    /// Generate a Pull Request Message for the changes made in this branch
    Pr,
    /// Review the Changes made in this branch against the base branch
    Review,
    /// Critique the changes made in this branch against the base branch
    Critique,
    /// Print all the configurations for debugging
    Debug,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub subcmd: SubCommand,

    #[clap(flatten)]
    pub config: LocalConfig,
}

impl Arguments {
    pub fn parse_all() -> Arguments {
        Arguments::parse()
    }
}
