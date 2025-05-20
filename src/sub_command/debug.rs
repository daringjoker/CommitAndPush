use crate::{args::Arguments, config::Config};

use super::SubCommand;

pub struct DebugCnp {}

impl SubCommand for DebugCnp {
    async fn run(&self, args: Arguments, config: Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Debugging Information:");
        println!("Config: {:?}", config);
        println!("=========================");
        println!("Arguments: {:?}", args);
        Ok(())
    }
}
