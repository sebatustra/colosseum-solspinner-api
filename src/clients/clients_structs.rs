use serde::Deserialize;
use std::collections::HashMap;

// BIRDEYE API

// TOKEN SECURITY

#[derive(Deserialize, Debug)]
pub struct ResponseSecurity {
    pub data: SecurityData,
    pub success: bool,
    #[serde(rename = "statusCode")]
    pub status_code: u16,
}

#[derive(Deserialize, Debug)]
pub struct SecurityData {
    #[serde(rename = "ownerAddress")]
    pub owner_address: Option<String>,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<String>,
}

// TOKEN OVERVIEW

#[derive(Deserialize, Debug)]
pub struct ResponseOverview {
    pub data: OverviewData,
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct OverviewData {
    #[serde(rename = "trade24h")]
    pub trade_24h: Option<u64>,
    pub decimals: i32,
    #[serde(rename = "priceChange24hPercent")]
    pub price_change_24h_percent: Option<f64>,
    #[serde(rename = "v24hUSD")]
    pub volume_24h_usd: Option<f64>,
    pub extensions: Option<OverviewExtensionData>
}

#[derive(Debug, Deserialize, Clone)]
pub struct OverviewExtensionData {
    pub discord: Option<String>,
    pub twitter: Option<String>,
    pub telegram: Option<String>,
    pub website: Option<String>
}

// TOKEN LIST

#[derive(Deserialize, Debug)]
pub struct ResponseTokens {
    pub data: TokenData,
}

#[derive(Deserialize, Debug)]
pub struct TokenData {
    pub tokens: Vec<TokenFromClient>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TokenFromClient {
    pub address: String,
    pub decimals: i32,
    pub liquidity: f64,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    #[serde(rename = "mc")]
    pub market_cap: f64,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "v24hUSD")]
    pub volume_24h_usd: f64,
}

// JUPITER API

#[derive(Deserialize, Debug)]
pub struct JupiterResponse {
    pub data: HashMap<String, JupiterTokenData>,
}

#[derive(Deserialize, Debug)]
pub struct JupiterTokenData {
    pub price: f64
}

