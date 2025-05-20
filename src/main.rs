mod ai;
mod args;
mod config;
mod sub_command;

use args::Arguments;
use config::Config;
use sub_command::dispatch_sub_command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments::parse_all();
    let mut config = Config::parse();

    if args.base_branch.is_some() {
        config.base_branch = args.base_branch.as_ref().unwrap().clone();
    }

    dispatch_sub_command(args, config).await
}
