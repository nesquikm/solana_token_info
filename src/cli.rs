use clap::Parser;

use crate::endpoint::Endpoint;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug_level: u8,

    /// Solana token address
    #[arg(short, long)]
    pub address: String,

    /// Solana network endpoint
    #[arg(short, long, default_value = "mainnet")]
    pub endpoint: Endpoint,
}

pub fn parse_cli() -> Cli {
    let cli = Cli::parse();

    cli
}
