use std::collections::HashMap;
use serde::Deserialize;
use crate::errors::Result;

pub struct JupiterClient;

#[derive(Debug)]
pub struct PriceUpdate {
    pub mint_pubkey: String,
    pub new_price: f64
}

impl JupiterClient {
    pub async fn get_tokens_price(token_mints: Vec<String>) -> Result<Vec<PriceUpdate>> {
        println!("->> {:<12} - get_tokens_price", "CLIENT");

        let mut query_url = String::from("https://price.jup.ag/v4/price?ids=");

        query_url.push_str(&token_mints.join(","));
        
        let request = reqwest::get(query_url)
            .await
            .map_err(|_| crate::errors::ApiError::JupiterFetchFail)?
            .json::<JupiterResponse>()
            .await
            .map_err(|_| crate::errors::ApiError::JupiterDeserializationFail)?;

        let mut price_updates: Vec<PriceUpdate> = Vec::with_capacity(token_mints.len());
        
        for mint_pubkey in request.data.keys() {
            price_updates.push(PriceUpdate {
                mint_pubkey: mint_pubkey.to_string(),
                new_price: request.data.get(mint_pubkey).unwrap().price
            })
        }

        Ok(price_updates)
    }
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct JupiterResponse {
    data: HashMap<String, TokenData>,
    _timeTaken: f64
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct TokenData {
    _id: String,
    _mintSymbol: String,
    _vsToken: String,
    _vsTokenSymbol: String,
    price: f64
}
