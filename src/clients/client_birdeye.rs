use axum::http::{HeaderMap, HeaderValue};
use reqwest::Client;
use crate::errors::{Result, ApiError};
use super::clients_structs::{ResponseSecurity, ResponseOverview, ResponseTokens};

pub struct BirdeyeClient {
    pub client: reqwest::Client
}

impl BirdeyeClient {
    pub fn new(birdeye_api_key: &str) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-API-KEY", 
            HeaderValue::from_str(birdeye_api_key).expect("Failed to add header auth for birdeye")
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to birdeye build client with headers");

        Self { client }
    }
}

impl BirdeyeClient {
    pub async fn get_tokens_list(&self, page: u32) -> Result<ResponseTokens> {
        println!("->> {:<12} - get_tokens_list", "CLIENT");

        let query_url = format!(
            "https://public-api.birdeye.so/defi/tokenlist?sort_by=v24hUSD&sort_type=desc&offset={}&limit=50",
            page
        );

        let response = self.client.get(query_url).send()
            .await
            .map_err(|e| {
                println!("Birdeye client failed fetching data. Error: {}", e);
                ApiError::BirdeyeFetchFail
            })?
            .json::<ResponseTokens>()
            .await
            .map_err(|e| {
                println!("Birdeye client failed deserializing data. Error: {}", e);
                ApiError::BirdeyeDeserializationFail
            })?;
            
        Ok(response)
    }

    pub async fn get_token_security(&self, token_pubkey: &str) -> Result<ResponseSecurity> {
        println!("->> {:<12} - get_token_security", "CLIENT");

        let query_url = format!(
            "https://public-api.birdeye.so/defi/token_security?address={}",
            token_pubkey
        );

        let response = self.client.get(query_url).send()
            .await
            .map_err(|e| {
                println!("Birdeye client failed fetching data. Error: {}", e);
                ApiError::BirdeyeFetchFail
            })?
            .json::<ResponseSecurity>()
            .await
            .map_err(|e| {
                println!("Birdeye client failed deserializing data. Error: {}", e);
                ApiError::BirdeyeDeserializationFail
            })?;

        Ok(response)
    }

    pub async fn get_token_overview(&self, token_pubkey: &str) -> Result<ResponseOverview> {
        println!("->> {:<12} - get_token_overview", "CLIENT");

        let query_url = format!(
            "https://public-api.birdeye.so/defi/token_overview?address={}",
            token_pubkey
        );

        let response = self.client.get(query_url).send()
            .await
            .map_err(|e| {
                println!("Birdeye client failed fetching data. Error: {}", e);
                ApiError::BirdeyeFetchFail
            })?
            .json::<ResponseOverview>()
            .await
            .map_err(|e| {
                println!("Birdeye client failed deserializing data. Error: {}", e);
                ApiError::BirdeyeDeserializationFail
            })?;

        Ok(response)
    }
}
