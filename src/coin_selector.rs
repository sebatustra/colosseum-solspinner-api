use std::collections::HashSet;

use crate::{clients::{client_birdeye::BirdeyeClient, clients_structs::TokenFromClient}, AppState};

pub struct CoinSelector;

impl CoinSelector {
    pub async fn run_coin_selection(state: AppState) {
        println!("->> {:<12} - run_coin_selection", "SELECTOR");

        let excluded_addresses: HashSet<&str> = [
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
        ].iter().copied().collect();

        let birdeye_client = BirdeyeClient::new(&state.birdeye_api_key);
    
        let list_response_1 = birdeye_client.get_tokens_list(1).await.unwrap();
        let list_response_2 = birdeye_client.get_tokens_list(2).await.unwrap();

        let list_tokens: Vec<TokenFromClient> = list_response_1.data.tokens
            .iter()
            .chain(list_response_2.data.tokens.iter())
            .cloned()
            .collect();
        
        let partially_filtered_tokens: Vec<TokenFromClient> = list_tokens
            .into_iter()
            .filter(|token| {
                token.market_cap >= 500_000.0 
                && token.liquidity >= 100_000.0
                && !excluded_addresses.contains(token.address.as_str())
            })
            .collect();

        println!("number of partially_filtered_tokens: {}", partially_filtered_tokens.len());

        let mut fully_filtered_tokens: Vec<TokenFromClient> = Vec::new();
    
        for mut token in partially_filtered_tokens {
             match birdeye_client.get_token_overview(&token.address).await {
                Ok(token_overview) => {
                    if token_overview.data.trade_24h >= 500 {
                        token.price_change_24h_percent = token_overview.data.price_change_24h_percent;
                        match birdeye_client.get_token_security(&token.address).await {
                            Ok(token_security) => {
                                if token_security.data.owner_address.is_none() && token_security.data.freeze_authority.is_none() {
                                    fully_filtered_tokens.push(token)
                                }
                            },
                            Err(_) => ()
                        }
                    }
                },
                Err(_) => ()
             }
        }

        println!("number of fully_filtered_tokens: {}", fully_filtered_tokens.len());

        println!("{:#?}", fully_filtered_tokens)

        // create the tokens table

        // delete current_tokens table data

        // update current_tokens_table

    }
}

