use serde::{Deserialize, Serialize};
use crate::{errors::api_errors::{ApiError, Result}, AppState};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Token {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Serialize, Clone)]
pub struct TokenForClient {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub price_change_24h_percent: f64,
}

impl TokenForClient {
    pub fn from_token(
        token: Token,
        price_change_24h_percent: f64
    ) -> Self {
        Self {
            mint_pubkey: token.mint_pubkey,
            symbol: token.symbol,
            name: token.name,
            logo_url: token.logo_url,
            price_change_24h_percent
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TokenForCreate {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
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
                "INSERT INTO tokens (mint_pubkey, symbol, name, logo_url, is_active) VALUES ($1, $2, $3, $4, $5) RETURNING *"
            )
            .bind(token.mint_pubkey)
            .bind(token.symbol)
            .bind(token.name)
            .bind(token.logo_url)
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
    ) -> Result<Vec<TokenForClient>> {
        println!("->> {:<12} - get_all_active_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE is_active = true"
        )
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(result) => {
                let mut tokens_for_client = Vec::new();

                for token in result {
                    // fetch data!!
                    let change = 0.12;
                    tokens_for_client.push(TokenForClient::from_token(token, change))
                }

                Ok(tokens_for_client)
            },
            Err(e) => {
                println!("Error fetching active tokens. Error: {}", e);
                Err(ApiError::TokenGetFail)
            }
        }
    }

    pub async fn get_7_active_tokens(
        state: AppState
    ) -> Result<Vec<TokenForClient>> {
        println!("->> {:<12} - get_7_active_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE is_active = true"
        )
        .fetch_all(&state.db)
        .await;

        match result {
            Ok(result) => {
                let mut tokens_for_client = Vec::new();

                for token in result {
                    // fetch data!!
                    let change = 0.12;
                    tokens_for_client.push(TokenForClient::from_token(token, change))
                }

                Ok(tokens_for_client)
            },
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
}

