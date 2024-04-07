use serde::{Deserialize, Serialize};
use crate::{errors::{ApiError, Result}, AppState};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Token {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Deserialize, Debug)]
pub struct TokenForCreate {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
}

// CRUD implementation for Token

impl Token {
    pub async fn create_token(
        token: TokenForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_token", "CONTROLLER");

        let token_exists = sqlx::query(
                "SELECT 1 FROM tokens WHERE mint_pubkey = $1"
            )
                .bind(&token.mint_pubkey)
                .fetch_optional(&state.db)
                .await
                .unwrap();

        if token_exists.is_some() {
            println!("Token with mint_pubkey {} already exists.", &token.mint_pubkey);
            return Err(ApiError::TokenAlreadyExists)
        }
        
        let result = sqlx::query_as::<_, Token>(
                "INSERT INTO tokens (mint_pubkey, symbol, name, logo_url) VALUES ($1, $2, $3, $4) RETURNING *"
            )
            .bind(token.mint_pubkey)
            .bind(token.symbol)
            .bind(token.name)
            .bind(token.logo_url)
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
}

