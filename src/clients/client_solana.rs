use std::str::FromStr;
use crate::errors::solana_errors::{Result, SolanaError};

use solana_client::rpc_client::RpcClient;
use spl_associated_token_account::{get_associated_token_address, create_associated_token_account};
use solana_sdk::{
    commitment_config::CommitmentConfig, 
    pubkey::Pubkey, 
    signer::{keypair::Keypair, Signer}, 
    transaction::Transaction
};

pub struct SolanaClient {
    pub rpc_client: RpcClient,
    pub payer_keypair: Keypair,
    pub comission_pubkey: String
}

impl SolanaClient {
    pub fn new(rpc_url: &str , keypair_str: &str, comission_pubkey: String) -> Self {
        SolanaClient { 
            rpc_client: RpcClient::new(rpc_url),
            payer_keypair: Keypair::from_base58_string(keypair_str),
            comission_pubkey
        }
    }
}

impl SolanaClient{
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

    pub fn create_comission_ata(&self, token_mint_address: &str) -> Result<()> {
        let user_pubkey = Pubkey::from_str(&self.comission_pubkey).map_err(|e| {
            println!("Error parsing pubkey. Error: {}", e);
            SolanaError::PubkeyParsingFail
        })?;

        let token_mint_pubkey = Pubkey::from_str(token_mint_address).map_err(|e| {
            println!("Error parsing pubkey. Error: {}", e);
            SolanaError::PubkeyParsingFail
        })?;

        let instruction = create_associated_token_account(
            &self.payer_keypair.pubkey(), 
            &user_pubkey, 
            &token_mint_pubkey
        );

        let latest_blockhash = self.rpc_client.get_latest_blockhash().map_err(|e| {
            println!("Error getting latest blockhash. Error: {}", e);
            SolanaError::GetLatestBlockhasFail
        })?;

        let transaction = Transaction::new_signed_with_payer(
            &[instruction], 
            Some(&self.payer_keypair.pubkey()), 
            &[&self.payer_keypair as &dyn Signer], 
            latest_blockhash
        );

        let trx_signature = self.rpc_client.send_and_confirm_transaction(&transaction).map_err(|e| {
            println!("Error sending and confirming transaction. Error: {}", e);
            SolanaError::SendTransactionFail
        })?;

        println!("trx_signature: {}", trx_signature);

        Ok(())
    }
}