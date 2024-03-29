use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{clients::client_jupiter::PriceUpdate, errors::{ApiError, Result}, AppState};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Token {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Deserialize, Debug)]
pub struct TokenForCreate {
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub price: f64,
}

// CRUD implementation for Token

impl Token {
    pub async fn create_token(
        token: TokenForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_token", "CONTROLLER");
        
        let result = sqlx::query_as::<_, Token>(
                "INSERT INTO tokens (mint_pubkey, symbol, name, price, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
            )
            .bind(token.mint_pubkey)
            .bind(token.symbol)
            .bind(token.name)
            .bind(token.price)
            .bind(Utc::now())
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

    pub async fn update_token_price(
        price_update: PriceUpdate, 
        state: &AppState
    ) -> Result<Token> {
        println!("->> {:<12} - update_token_price", "CONTROLLER");

        let result = sqlx::query_as::<_, Token>(
                "UPDATE tokens SET price = $1, updated_at = $2 WHERE mint_pubkey = $3 RETURNING *"
            )
            .bind(price_update.new_price)
            .bind(Utc::now())
            .bind(&price_update.mint_pubkey)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(token) => Ok(token),
            Err(e) => {
                println!(
                    "Error updating mint: {}. Error: {}",
                    price_update.mint_pubkey,
                    e
                );
                Err(ApiError::TokenUpdateFail)
            }
        }
    }
}

