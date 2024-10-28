use anyhow::{anyhow, Ok, Result};
use mpl_token_metadata::accounts::Metadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

#[derive(Debug)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub async fn get_token_metadata(rpc: &RpcClient, pubkey: &Pubkey) -> Result<TokenMetadata> {
    let (metadata_pda, _) = Metadata::find_pda(&pubkey);

    let metadata_data = rpc
        .get_account_data(&metadata_pda)
        .await
        .map_err(|e| anyhow!("Error fetching metadata account: {}", e))?;

    let metadata = mpl_token_metadata::accounts::Metadata::safe_deserialize(&metadata_data)
        .map_err(|e| anyhow!("Error deserializing metadata: {}", e))?;

    Ok(TokenMetadata {
        name: metadata.name.trim_end_matches('\0').to_owned(),
        symbol: metadata.symbol.trim_end_matches('\0').to_owned(),
        uri: metadata.uri.trim_end_matches('\0').to_owned(),
    })
}
