use std::str::FromStr;
use crate::errors::solana_errors::{Result, SolanaError};

use solana_client::rpc_client::RpcClient;
use spl_associated_token_account::get_associated_token_address;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

pub struct SolanaClient {
    pub rpc_client: RpcClient
}

impl SolanaClient {
    pub fn new(url: String) -> Self {
        SolanaClient { 
            rpc_client: RpcClient::new(url)
        }
    }
}

impl SolanaClient {
    pub fn check_user_has_ata(&self, user_pubkey: &str, token_mint_address: &str) -> Result<bool>{

        let user_pubkey = Pubkey::from_str(user_pubkey).map_err(|e| {
            println!("Error parsing pubkey. Error: {}", e);
            SolanaError::PubkeyParsingFail
        })?;

        let token_mint_pubkey = Pubkey::from_str(token_mint_address).map_err(|e| {
            println!("Error parsing pubkey. Error: {}", e);
            SolanaError::PubkeyParsingFail
        })?;

        let user_ata = get_associated_token_address(&user_pubkey, &token_mint_pubkey);

        match self.rpc_client.get_account_with_commitment(&user_ata, CommitmentConfig::confirmed()) {
            Ok(account_info) => {
                let response = account_info.value;
                match response {
                    Some(_) => Ok(true),
                    None => Ok(false)
                }
            },
            Err(error) => {
                println!("Error getting account info. Error: {}", error);
                Err(SolanaError::GetAccountInfoFail)
            }
        }

    }
}