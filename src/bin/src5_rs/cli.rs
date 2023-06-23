use crate::commands::Parse;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "src5")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[clap(about = "Parse a file generating interface signatures for each trait")]
    Parse(Parse),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
