use anyhow::{anyhow, Ok, Result};
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use solana_program::pubkey::Pubkey;

const TOKEN_LIST_URL: &str = "https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json";

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct TagDetails {
    name: String,
    description: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct TokenList {
    name: String,
    #[serde(rename = "logoURI")]
    logo_uri: String,
    tags: HashMap<String, TagDetails>,
    timestamp: String,
    tokens: Vec<TokenInfo>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]

pub struct TokenExtensions {
    website: Option<String>,
    bridge_contract: Option<String>,
    asset_contract: Option<String>,
    address: Option<String>,
    explorer: Option<String>,
    twitter: Option<String>,
    github: Option<String>,
    medium: Option<String>,
    tgann: Option<String>,
    tggroup: Option<String>,
    discord: Option<String>,
    serum_v3_usdt: Option<String>,
    serum_v3_usdc: Option<String>,
    coingecko_id: Option<String>,
    image_url: Option<String>,
    description: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    chain_id: u32,
    address: String,
    name: String,
    decimals: u8,
    symbol: String,
    logo_uri: Option<String>,
    tags: Option<Vec<String>>,
    extensions: Option<TokenExtensions>,
}

async fn fetch_token_list() -> Result<TokenList> {
    let response = get(TOKEN_LIST_URL).await?.text().await?;
    let token_list: TokenList = serde_json::from_str(&response)?;
    Ok(token_list)
}

pub async fn get_token_info(address: &Pubkey) -> Result<TokenInfo> {
    let token_list = fetch_token_list().await?;
    let address_string = &address.to_string();
    let token_info = token_list
        .tokens
        .iter()
        .find(|token| &token.address == address_string);
    match token_info {
        Some(token_info) => Ok(token_info.clone()),
        None => Err(anyhow!("Token not found")),
    }
}

pub async fn get_token_website(address: &Pubkey) -> Result<String> {
    let token_info = get_token_info(address).await?;
    match token_info.extensions {
        Some(extensions) => match extensions.website {
            Some(website) => Ok(website),
            None => Err(anyhow!("Website not found")),
        },
        None => Err(anyhow!("Extensions not found")),
    }
}
