use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use anyhow::{anyhow, Ok, Result};

pub async fn get_token_supply(rpc: &RpcClient, pubkey: &Pubkey) -> Result<String> {
    let token_supply = rpc
        .get_token_supply(pubkey).await.map_err(|e| anyhow!("Error fetching token supply: {}", e))?;
    Ok(token_supply.ui_amount_string)
}
