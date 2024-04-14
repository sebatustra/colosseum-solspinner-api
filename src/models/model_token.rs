use serde::{Deserialize, Serialize};
use crate::{errors::api_errors::{ApiError, Result}, AppState};

#[derive(Debug, sqlx::FromRow, Serialize, Clone)]
pub struct Token {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub price_change_24h_percent: f64,
    pub volume_24h_usd: f64,
    pub discord_url: Option<String>,
    pub twitter_url: Option<String>,
    pub website_url: Option<String>,
    pub telegram_url: Option<String>,
    pub decimals: i32,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Deserialize, Debug)]
pub struct TokenForCreate {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub price_change_24h_percent: f64,
    pub volume_24h_usd: f64,
    pub discord_url: Option<String>,
    pub twitter_url: Option<String>,
    pub website_url: Option<String>,
    pub telegram_url: Option<String>,
    pub decimals: i32,
    pub is_active: bool
}

#[derive(Debug, sqlx::FromRow)]
pub struct TokenPubkey {
    pub mint_pubkey: String
}

// CRUD implementation for Token

impl Token {
    pub async fn create_token(
        token: TokenForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_token", "CONTROLLER");
        
        let result = sqlx::query_as::<_, Token>(
                r#"INSERT INTO tokens 
                (mint_pubkey, symbol, name, logo_url, price_change_24h_percent, volume_24h_usd, discord_url, twitter_url, website_url, telegram_url, decimals, is_active) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) 
                RETURNING *"#
            )
            .bind(token.mint_pubkey)
            .bind(token.symbol)
            .bind(token.name)
            .bind(token.logo_url)
            .bind(token.price_change_24h_percent)
            .bind(token.volume_24h_usd)
            .bind(token.discord_url)
            .bind(token.twitter_url)
            .bind(token.website_url)
            .bind(token.telegram_url)
            .bind(token.decimals)
            .bind(token.is_active)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(token) => Ok(token),
            Err(e) => {
                println!("Error creating token. Error: {}", e);
                Err(ApiError::TokenCreateFail)
            }
        }
    }

    pub async fn get_tokens(state: AppState) -> Result<Vec<Token>> {
        println!("->> {:<12} - get_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
                "SELECT * FROM tokens"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                println!("Error fetching tokens. Error: {}", e);
                Err(ApiError::TokenGetFail)
            }
        }
    }

    pub async fn get_token(
        mint_pubkey: &str, 
        state: AppState
    ) -> Result<Option<Token>> {
        println!("->> {:<12} - get_token", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE mint_pubkey = $1"
        )
        .bind(mint_pubkey)
        .fetch_optional(&state.db)
        .await;

        match result {
            Ok(result) => Ok(result),
            Err(e) => {
                println!("Error fetching token. Error: {}", e);
                Err(ApiError::TokenGetFail)
            }
        }
    }

    pub async fn get_all_active_tokens(
        state: AppState
    ) -> Result<Vec<Token>> {
        println!("->> {:<12} - get_all_active_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE is_active = true"
        )
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(tokens) => Ok(tokens)
            ,
            Err(e) => {
                println!("Error fetching active tokens. Error: {}", e);
                Err(ApiError::TokenGetFail)
            }
        }
    }

    pub async fn get_7_active_tokens(
        state: AppState
    ) -> Result<Vec<Token>> {
        println!("->> {:<12} - get_7_active_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
            r#"SELECT * 
                FROM tokens 
                WHERE is_active = true
                ORDER BY volume_24h_usd DESC
                LIMIT 7"#
        )
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                println!("Error fetching active tokens. Error: {}", e);
                Err(ApiError::TokenGetFail)
            }
        }
    }

    pub async fn update_token_state(
        mint_pubkey: &str,
        new_state: bool,
        state: AppState
    ) -> Result<()> {
        println!("->> {:<12} - update_token_state", "CONTROLLER");

        let result = sqlx::query(
            "UPDATE tokens SET is_active = $1 WHERE mint_pubkey = $2"
        )
        .bind(new_state)
        .bind(mint_pubkey)
        .execute(&state.db)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error updating token is_active column. Error: {}", e);
                Err(ApiError::TokenUpdateFail)
            }
        }
    }

    pub async fn update_token_financial_data(
        mint_pubkey: &str,
        price_change_24h_percent: f64,
        volume_24h_usd: f64,
        decimals: i32,
        state: AppState
    ) -> Result<()> {
        println!("->> {:<12} - update_token_state", "CONTROLLER");

        let result = sqlx::query(
            r#"UPDATE tokens 
            SET 
                price_change_24h_percent = $1,
                volume_24h_usd = $2,
                decimals = $3
            WHERE mint_pubkey = $4"#
        )
        .bind(price_change_24h_percent)
        .bind(volume_24h_usd)
        .bind(decimals)
        .bind(mint_pubkey)
        .execute(&state.db)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error updating token financial data. Error: {}", e);
                Err(ApiError::TokenUpdateFail)
            }
        }
    }
}

