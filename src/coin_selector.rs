use std::collections::HashSet;

use crate::{
    clients::{client_birdeye::BirdeyeClient, clients_structs::TokenFromClient}, 
    errors::cron_errors::{CronError, Result}, 
    models::model_token::{Token, TokenForCreate}, 
    AppState
};

pub struct CoinSelector;

impl CoinSelector {
    pub async fn run_coin_selection(state: AppState) -> Result<()> {
        println!("->> {:<12} - run_coin_selection", "SELECTOR");

        let birdeye_client = BirdeyeClient::new(&state.birdeye_api_key);

        let excluded_addresses = get_excluded_addresses();
    
        let list_response_1 = birdeye_client.get_tokens_list(1)
            .await.map_err(|_| CronError::BirdeyeClientFail)?;

        let list_response_2 = birdeye_client.get_tokens_list(2)
            .await.map_err(|_| CronError::BirdeyeClientFail)?;

        let token_list = join_token_lists(
            list_response_1.data.tokens, 
            list_response_2.data.tokens
        );
        
        let partially_filtered_tokens = filter_by_mc_liquidity_and_addresses(
            token_list, 
            excluded_addresses
        )?;

        let fully_filtered_tokens = filter_by_24htrade_and_security(
            partially_filtered_tokens, 
            birdeye_client
        ).await?;

        let current_active_tokens_pubkey: Vec<String> = get_current_active_pubkeys(state.clone()).await?;

        update_or_create_tokens(
            fully_filtered_tokens, 
            current_active_tokens_pubkey, 
            state.clone()
        ).await?;

        Ok(())
    }
}

async fn update_or_create_tokens(
    token_list: Vec<TokenFromClient>,
    current_active_tokens_pubkey: Vec<String>,
    state: AppState
) -> Result<()> {
    for token in token_list {
        if current_active_tokens_pubkey.contains(&token.address) {
            continue;
        } else {
            match Token::get_token(&token.address, state.clone()
            )
            .await
            .map_err(|_| CronError::UpdateTokenStatusFail)? {
                Some(token) => {
                    Token::update_token_state(
                        &token.mint_pubkey, 
                        true, 
                        state.clone()
                    )
                    .await
                    .map_err(|_| CronError::UpdateTokenStatusFail)?
                },
                None => {
                    let new_token = TokenForCreate {
                        mint_pubkey: token.address,
                        symbol: token.symbol.clone(),
                        name: token.name.clone(),
                        logo_url: token.logo_uri.clone(),
                        is_active: true
                    };

                    Token::create_token(
                        new_token, 
                        state.clone()
                    )
                    .await
                    .map_err(|_| CronError::UpdateTokenStatusFail)?;
                }
            }
        }
    }

    Ok(())
}

async fn get_current_active_pubkeys(
    state: AppState
) -> Result<Vec<String>> {
        Ok(
            Token::get_all_active_tokens(state.clone())
                .await
                .map_err(|_| CronError::UpdateTokenStatusFail)?
                .into_iter()
                .map(|token| token.mint_pubkey)
                .collect()
        )
}

fn join_token_lists(
    token_list_1: Vec<TokenFromClient>,
    token_list_2: Vec<TokenFromClient>
) -> Vec<TokenFromClient> {
    token_list_1
        .into_iter()
        .chain(token_list_2.into_iter())
        .collect()
}

fn filter_by_mc_liquidity_and_addresses(
    token_list: Vec<TokenFromClient>,
    excluded_addresses: HashSet<String>
) -> Result<Vec<TokenFromClient>> {
    let filtered_token_list: Vec<TokenFromClient> = token_list.into_iter()
        .filter(|token| {
            token.market_cap >= 500_000.0 
            && token.liquidity >= 100_000.0
            && !excluded_addresses.contains(token.address.as_str())
        })
        .collect();

    if filtered_token_list.len() < 25 {
        Err(CronError::FilteredTokensLengthFail)
    } else {
        Ok(filtered_token_list)
    }
}

async fn filter_by_24htrade_and_security(
    token_list: Vec<TokenFromClient>,
    birdeye_client: BirdeyeClient
) -> Result<Vec<TokenFromClient>> {
    let mut fully_filtered_tokens = Vec::new();

    for mut token in token_list {
        let token_overview = birdeye_client.get_token_overview(&token.address)
            .await.map_err(|_| CronError::BirdeyeClientFail)?;
            
        if token_overview.data.trade_24h >= 500 {
            token.price_change_24h_percent = token_overview.data.price_change_24h_percent;

            let token_security =  birdeye_client.get_token_security(&token.address)
                .await.map_err(|_| CronError::BirdeyeClientFail)?;

            if token_security.data.owner_address.is_none() && token_security.data.freeze_authority.is_none() {
                fully_filtered_tokens.push(token)
            }
        }
   }

   if fully_filtered_tokens.len() < 25 {
        return Err(CronError::FilteredTokensLengthFail)
   } else {
       Ok(fully_filtered_tokens.drain(0..25).collect())
   }
}

fn get_excluded_addresses() -> HashSet<String> {
    vec![
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
        "7kbnvuGBxxj8AG9qp8Scn56muWGaRaFqxg1FsRp3PaFT",
        "So11111111111111111111111111111111111111112",
        "JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN",
        "bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1",
        "J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn",
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So",
        "7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj",
        "4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R",
        "ZEUS1aR7aX8DFFJf5QjWj2ftDDdNTroMNGo8YoQm3Gq",
        "jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL",
        "85VBFQZC9TZkfaptBWjvUw7YbZjy52A6mjtPGjstQAmQ",
        "HZ1JovNiVvGrGNiiYvEozEVgZ58xaU3RKwX8eACQBCt3",
        "27G8MtK7VtTcCHkpASjSDdkWWYfoqT6ggEuKidVJidD4",
        "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs",
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh",
        "rndrizKT3MK1iimdxRdWabcF7Zg7AR5T4nud4EkHBof",
        "SHDWyBxihqiCj6YekG2GUr7wqKLeLAMK1gHZck9pL6y",
        "LFNTYraetVioAPnGJht4yNg2aUZFXR776cMeN9VMjXp",
        "orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE",
        "hntyVP6YFm1Hg25TN9WGLqM12b8TQmcknKrdu1oxWux",
        "nosXBVoaCTtYdLvKY6Csb4AC8JCdQKKAaWYtx2ZMoo7",
        "mb1eu7TzEc71KxDpsmsKoucSSuuoGLv1drys1oP2jh6",
        "SLNDpmoWTVADgEdndyvWzroNL7zSi1dF9PC3xHGtPwp",
        "ATLASXmbPQxBUYbxPsV97usA3fPQYEqzQBUHgiFCUsXx"
    ]
    .iter()
    .map(|&pubkey| pubkey.to_string())
    .collect()
}