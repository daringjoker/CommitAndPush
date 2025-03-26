use crate::args::{Arguments, SubCommand as SubCommandTypes};
use crate::config::Config;
mod critique;
mod pr;
mod review;

use critique::Critique;
use pr::PullRequest;
use review::Review;

pub trait SubCommand {
    async fn run(&self, args: Arguments, config: Config) -> Result<(), Box<dyn std::error::Error>>;
}

pub async fn dispatch_sub_command(
    args: Arguments,
    config: Config,
) -> Result<(), Box<dyn std::error::Error>> {
    match args.subcmd {
        SubCommandTypes::Pr => {
            let pr = PullRequest {};
            pr.run(args, config).await
        }

        SubCommandTypes::Review => {
            let review = Review {};
            review.run(args, config).await
        }

        SubCommandTypes::Critique => {
            let critique = Critique {};
            critique.run(args, config).await
        }
    }
}
