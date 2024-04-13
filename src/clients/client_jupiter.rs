
use crate::errors::api_errors::{Result, ApiError};
use super::clients_structs::JupiterResponse;
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

        let response = reqwest::get(query_url)
            .await
            .map_err(|e| {
                println!("Jupiter client failed fetching data. Error: {}", e);
                ApiError::JupiterFetchFail
            })?
            .json::<JupiterResponse>()
            .await
            .map_err(|e| {
                println!("Jupiter client failed deserializing data. Error: {}", e);
                ApiError::JupiterDeserializationFail
            })?;

        
        let new_price = response.data.get(token_pubkey).unwrap().price;

        Ok(new_price)
    }
}
