use crate::clients::clients_structs::TokenFromClient;


#[derive(Debug)]
pub struct TokenForCron {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub logo_uri: String,
    pub price_change_24h_percent: f64,
    pub volume_24h_usd: f64,
    pub discord: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
    pub telegram: Option<String>,
    pub decimals: i32
}

impl TokenForCron {
    pub fn create_from_client_token(client_token: TokenFromClient) -> Self {
        Self {
            address: client_token.address,
            symbol: client_token.symbol,
            name: client_token.name,
            logo_uri: client_token.logo_uri.unwrap_or("".to_string()),
            volume_24h_usd: client_token.volume_24h_usd,
            price_change_24h_percent: 0.0,
            discord: None,
            twitter: None,
            website: None,
            telegram: None,
            decimals: 0,
        }
    }
}