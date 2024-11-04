use anyhow::{anyhow, Ok, Result};
use log::info;
use mpl_token_metadata::accounts::Metadata;
use solana_account_decoder::parse_token::{parse_token, TokenAccountType};
use solana_account_decoder::parse_token_extension::{UiExtension, UiTokenMetadata};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

#[derive(Debug)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

pub async fn get_token_metadata(rpc: &RpcClient, pubkey: &Pubkey) -> Result<TokenMetadata> {
    let account_data = rpc
        .get_account_data(&pubkey)
        .await
        .map_err(|e| anyhow!("Error fetching account data: {}", e))?;

    let token =
        parse_token(&account_data, None).map_err(|e| anyhow!("Error parsing token: {}", e))?;

    // Getting token metadata from SPL token account (Token-2022)
    match token {
        TokenAccountType::Mint(mint) => {
            let metadata = mint.extensions.iter().find(|ext| {
                if let UiExtension::TokenMetadata(UiTokenMetadata {
                    update_authority: _,
                    mint: _,
                    name: _,
                    symbol: _,
                    uri: _,
                    additional_metadata: _,
                }) = ext
                {
                    return true;
                }
                return false;
            });

            match metadata {
                Some(UiExtension::TokenMetadata(UiTokenMetadata {
                    update_authority: _,
                    mint: _,
                    name,
                    symbol,
                    uri,
                    additional_metadata: _,
                })) => {
                    info!("Token metadata found in SPL token account");
                    return Ok(TokenMetadata {
                        name: name.trim_end_matches('\0').to_owned(),
                        symbol: symbol.trim_end_matches('\0').to_owned(),
                        uri: uri.trim_end_matches('\0').to_owned(),
                    });
                }
                _ => {}
            }
        }
        _ => {}
    }

    info!("Token metadata not found in SPL token account, trying to fetch from metadata account");

    // Okay, we don't have metadata in the SPL token account, let's try to fetch it from the
    // metadata account (Metaplex Token Metadata)

    let (metadata_pda, _) = Metadata::find_pda(&pubkey);

    let metadata_data = rpc
        .get_account_data(&metadata_pda)
        .await
        .map_err(|e| anyhow!("Error fetching metadata account: {}", e))?;

    let metadata = mpl_token_metadata::accounts::Metadata::safe_deserialize(&metadata_data)
        .map_err(|e| anyhow!("Error deserializing metadata: {}", e))?;

    info!("Token metadata found in metadata account");

    Ok(TokenMetadata {
        name: metadata.name.trim_end_matches('\0').to_owned(),
        symbol: metadata.symbol.trim_end_matches('\0').to_owned(),
        uri: metadata.uri.trim_end_matches('\0').to_owned(),
    })
}
