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
    let config = Config::parse();
    dispatch_sub_command(args, config).await
}
