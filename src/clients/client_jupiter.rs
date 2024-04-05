use std::collections::HashMap;
use serde::Deserialize;
use crate::errors::Result;

pub struct JupiterClient;

impl JupiterClient {
    pub async fn get_token_price(
        token_pubkey: &str,
        vs_token_symbol: &str
    ) -> Result<f64> {
        println!("->> {:<12} - get_token_price", "CLIENT");

        let query_url = format!(
            "https://price.jup.ag/v4/price?ids={}&vsToken={}",
            token_pubkey,
            vs_token_symbol
        );

        let request = reqwest::get(query_url)
            .await
            .map_err(|e| {
                println!("Jupiter client failed fetching data. Error: {}", e);
                crate::errors::ApiError::JupiterFetchFail
            })?
            .json::<JupiterResponse>()
            .await
            .map_err(|e| {
                println!("Jupiter client failed serializing data. Error: {}", e);
                crate::errors::ApiError::JupiterDeserializationFail
            })?;

        
        let new_price = request.data.get(token_pubkey).unwrap().price;

        Ok(new_price)
    }
}

#[derive(Deserialize, Debug)]
struct JupiterResponse {
    data: HashMap<String, TokenData>,
}

#[derive(Deserialize, Debug)]
struct TokenData {
    price: f64
}
