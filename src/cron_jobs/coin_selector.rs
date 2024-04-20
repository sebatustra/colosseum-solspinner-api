use std::collections::HashSet;

use tokio_cron_scheduler::Job;

use crate::{
    clients::{client_birdeye::BirdeyeClient, clients_structs::TokenFromClient}, 
    errors::cron_errors::{CronError, Result}, 
    models::model_token::{Token, TokenForCreate}, 
    AppState
};

use super::cron_structs::TokenForCron;

pub struct CoinSelector;

impl CoinSelector {
    pub fn init_job(
        job_schedule: &str,
        state: AppState
    ) -> Job {
        Job::new_async(job_schedule, move |_, _| {
            let state_copy = state.clone();
            Box::pin(async move {
                let mut attempts = 0;

                while attempts < 3 {
                    match Self::run_coin_selection(
                        state_copy.clone(),
                    ).await {
                        Ok(_) => {
                            println!("->> {:<12} - run_coin_selection succeeded", "CRON");
                            break;
                        },
                        Err(e) => {
                            println!("->> {:<12} - run_coin_selection failed. Error: {e}", "CRON");
                            attempts += 1;

                            if attempts >= 3 {
                                println!("->> {:<12} - run_coin_selection failed 3 times", "CRON");
                                break;
                            }
                        }
                    }
                }
            })
        }).expect("Failed to add job")
    }
}

impl CoinSelector {
    pub async fn run_coin_selection(
        state: AppState, 
    ) -> Result<()> {
        println!("->> {:<12} - run_coin_selection", "SELECTOR");

        let birdeye_client = &state.birdeye_client;

        let excluded_addresses = get_excluded_addresses();
    
        let list_response_1 = birdeye_client.get_tokens_list(1)
            .await.map_err(|_| CronError::BirdeyeClientFail)?;

        let list_response_2 = birdeye_client.get_tokens_list(2)
            .await.map_err(|_| CronError::BirdeyeClientFail)?;

        let token_list = join_token_lists(
            list_response_1.data.tokens, 
            list_response_2.data.tokens,

        );
        
        let partially_filtered_tokens = filter_by_mc_liquidity_and_addresses(
            token_list, 
            excluded_addresses
        )?;

        let fully_filtered_tokens = filter_by_24htrade_and_security(
            partially_filtered_tokens, 
            birdeye_client
        ).await?;

        println!("fully_filtered_tokens lenght: {}", fully_filtered_tokens.len());

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
    token_list: Vec<TokenForCron>,
    current_active_tokens_pubkey: Vec<String>,
    state: AppState
) -> Result<()> {
    println!("length of tokens passed to update_or_create_tokens: {}", token_list.len());

    let token_list_pubkeys: HashSet<String> = token_list.iter().map(|token| token.address.clone()).collect();
    let current_active_set: HashSet<String> = current_active_tokens_pubkey.clone().into_iter().collect();

    for token in token_list {
        if current_active_tokens_pubkey.contains(&token.address) {
            println!("Token is currently active: {}", token.address);
            continue;
        } else {
            println!("Token is not currently active: {}", token.address);
            match Token::get_token(&token.address, state.clone()
            )
            .await
            .map_err(|_| CronError::UpdateTokenStatusFail)? {
                Some(token) => {
                    println!("Token exists... changing it to active: {}", &token.mint_pubkey);
                    Token::update_token_state(
                        &token.mint_pubkey, 
                        true, 
                        state.clone()
                    )
                    .await
                    .map_err(|_| CronError::UpdateTokenStatusFail)?
                },
                None => {
                    println!("Token does not exists... creating it now: {}", token.address);
                    let new_token = TokenForCreate {
                        mint_pubkey: token.address,
                        symbol: token.symbol.clone(),
                        name: token.name.clone(),
                        logo_url: token.logo_uri.clone(),
                        price_change_24h_percent: token.price_change_24h_percent,
                        volume_24h_usd: token.volume_24h_usd,
                        discord_url: token.discord,
                        twitter_url: token.twitter,
                        website_url: token.website,
                        telegram_url: token.telegram,
                        decimals: token.decimals,
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

    for pubkey in current_active_set.difference(&token_list_pubkeys) {
        println!("Token is no longer active: {}", pubkey);
        Token::update_token_state(
            pubkey,
             false, 
            state.clone()
        )
        .await.map_err(|_| CronError::UpdateTokenStatusFail)?;
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
    token_list_2: Vec<TokenFromClient>,
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
    birdeye_client: &BirdeyeClient
) -> Result<Vec<TokenForCron>> {
    let mut seen_pubkeys = HashSet::new();

    let mut fully_filtered_tokens = Vec::new();

    for token in token_list {
        if seen_pubkeys.insert(token.address.clone()) {
            let mut token_for_cron = TokenForCron::create_from_client_token(token);


            let token_overview = birdeye_client.get_token_overview(&token_for_cron.address)
                .await.map_err(|_| CronError::BirdeyeClientFail)?;
                
            if token_overview.data.trade_24h.unwrap_or(0) >= 500
            {
                token_for_cron.price_change_24h_percent = token_overview.data.price_change_24h_percent.unwrap_or(0.0);
                token_for_cron.decimals = token_overview.data.decimals;

                if token_overview.data.extensions.is_some() {
                    token_for_cron.discord = token_overview.data.extensions.clone().unwrap().discord;
                    token_for_cron.twitter = token_overview.data.extensions.clone().unwrap().twitter;
                    token_for_cron.telegram = token_overview.data.extensions.clone().unwrap().telegram;
                    token_for_cron.website = token_overview.data.extensions.unwrap().website;
                } else {
                    token_for_cron.discord = None;
                    token_for_cron.twitter = None;
                    token_for_cron.telegram = None;
                    token_for_cron.website = None;
                }

                let token_security =  birdeye_client.get_token_security(&token_for_cron.address)
                    .await.map_err(|_| CronError::BirdeyeClientFail)?;
    
                if token_security.data.owner_address.is_none() && token_security.data.freeze_authority.is_none() {
                    fully_filtered_tokens.push(token_for_cron)
                }
            }
        }
   }

   println!("the length is: {}", fully_filtered_tokens.len());

   if fully_filtered_tokens.len() < 25 {
        return Err(CronError::FilteredTokensLengthFail)
   } else {
        let drained_list: Vec<TokenForCron> =  fully_filtered_tokens.drain(0..25).collect();

        Ok(drained_list)
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