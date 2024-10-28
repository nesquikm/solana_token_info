use crate::endpoint::Endpoint;
use anyhow::{anyhow, Error, Ok, Result};
use log::{error, info};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::fmt;
use std::str::FromStr;
use tokio::try_join;

mod metadata;
mod supply;
mod tokenlist;
mod resolver;

#[derive(Debug)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub supply: String,
    pub uri: String,
    pub ipv4records: usize,
    pub ipv6records: usize,
}

impl fmt::Display for TokenInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Token info:\n   * Name: {}\n   * Symbol: {}\n   * Supply {}\n   * URI: {}\n   * IPv4 records: {}\n   * IPv6 records: {}",
            self.name, self.symbol, self.supply, self.uri, self.ipv4records, self.ipv6records
        )
    }
}

pub async fn get_info(endpoint: Endpoint, address: String) -> Result<TokenInfo> {
    info!("Requesting data from: {} ({})", endpoint, endpoint.url());

    let pubkey =
        Pubkey::from_str(&address).map_err(|e| error(&format!("Error parsing pubkey {}", e)))?;

    info!("Pubkey parsed: {:#?}", pubkey);

    let rpc = RpcClient::new(endpoint.url().to_string());

    let token_metadata = metadata::get_token_metadata(&rpc, &pubkey);

    let token_supply = supply::get_token_supply(&rpc, &pubkey);

    let token_website_from_tokenlist = tokenlist::get_token_website(&pubkey);

    let (mut token_metadata, token_supply) = try_join!(token_metadata, token_supply)
        .map_err(|e| error(&format!("Error fetching token metadata and supply: {}", e)))?;

    info!("Token metadata: {:?}", token_metadata);
    info!("Token supply: {}", token_supply);

    if token_metadata.uri.is_empty() {
        info!("Token metadata URI is empty, trying to fetch from tokenlist");
        let token_website_from_tokenlist = token_website_from_tokenlist.await.map_err(|e| {
            error(&format!(
                "Error fetching token website from tokenlist: {}",
                e
            ))
        })?;
        info!(
            "Token website from tokenlist: {:#?}",
            token_website_from_tokenlist
        );
        token_metadata.uri = token_website_from_tokenlist;
    }

    let dns_records = resolver::get_website_dns_records(&token_metadata.uri).await.map_err(|e| {
        error(&format!("Error fetching DNS records for website: {}", e))
    })?;

    Ok(TokenInfo {
        name: token_metadata.name,
        symbol: token_metadata.symbol,
        supply: token_supply,
        uri: token_metadata.uri,
        ipv4records: dns_records.ipv4count,
        ipv6records: dns_records.ipv6count,
    })
}

fn error(message: &str) -> Error {
    error!("{}", message);
    anyhow!("{}", message)
}
