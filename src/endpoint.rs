use std::fmt;

use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Endpoint {
  Mainnet,
  Devnet,
  Testnet,
}

impl fmt::Display for Endpoint {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Endpoint::Mainnet => write!(f, "mainnet"),
      Endpoint::Devnet => write!(f, "devnet"),
      Endpoint::Testnet => write!(f, "testnet"),
    }
  }
}

impl Endpoint {
  pub fn url(&self) -> &'static str {
    match self {
      Endpoint::Mainnet => "https://api.mainnet-beta.solana.com",
      Endpoint::Devnet => "https://api.devnet.solana.com",
      Endpoint::Testnet => "https://api.testnet.solana.com",
    }
  }
}
