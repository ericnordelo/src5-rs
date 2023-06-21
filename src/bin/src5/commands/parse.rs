use anyhow::{Ok, Result};
use async_trait::async_trait;
use clap::Parser;

use super::CliCommand;

#[derive(Parser, Debug)]
pub struct Parse {
    #[clap(help = "File path to the Cairo source code", long, short)]
    pub cairo_path: String,
}

#[async_trait]
impl CliCommand for Parse {
    // Parse a file generating interface signatures for each trait
    async fn run(&self) -> Result<()> {
        println!("Parsing...");
        Ok(())
    }
}
