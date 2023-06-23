mod cli;
mod commands;

use anyhow::{Ok, Result};
use dotenv::dotenv;

use clap::Parser;

use cli::Cli;
use commands::CliCommand;

#[tokio::main]
async fn main() -> Result<()> {
    // Load the environment variables from the ".env" file
    dotenv().ok();

    let cli = Cli::parse();
    match cli.command {
        cli::Commands::Parse(cmd) => {
            cmd.run().await?;
        }
    };
    Ok(())
}
