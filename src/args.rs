use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand, Clone)]
// #[Subcommand(rename_all = "lower")]
pub enum SubCommand {
    /// Generate a Pull Request Message for the changes made in this branch
    Pr,
    /// Review the Changes made in this branch against the base branch
    Review,
    /// Critique the changes made in this branch against the base branch
    Critique,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub subcmd: SubCommand,
    #[clap(long = "base-branch", short = 'b')]
    pub base_branch: Option<String>,
}

impl Arguments {
    pub fn parse_all() -> Arguments {
        Arguments::parse()
    }
}
