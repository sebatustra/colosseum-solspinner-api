use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{errors::{ApiError, Result}, AppState};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Position {
    pub id: Uuid,
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub mint_symbol: String,
    pub vs_token_symbol: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct PositionWithProfit {
    pub id: Uuid,
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub mint_symbol: String,
    pub vs_token_symbol: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub current_price: f64,
    pub percentage_change: f64,
    pub price_change: f64
}

impl PositionWithProfit {
    pub fn new(
        position: Position, 
        current_price: f64,
        percentage_change: f64, 
        price_change: f64
    ) -> Self {
        PositionWithProfit {
            id: position.id,
            user_pubkey: position.user_pubkey,
            mint_pubkey: position.mint_pubkey,
            mint_symbol: position.mint_symbol,
            vs_token_symbol: position.vs_token_symbol,
            quantity: position.quantity,
            purchase_price: position.purchase_price,
            created_at: position.created_at,
            current_price,
            percentage_change,
            price_change
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PositionForCreate {
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub mint_symbol: String,
    pub vs_token_symbol: String,
    pub quantity: f64,
    pub purchase_price: f64,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct PositionMint {
    pub mint_pubkey: String
}

// CRUD implementation for Position

impl Position {
    pub async fn create_position(
        position: PositionForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_position", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "INSERT INTO positions (user_pubkey, mint_pubkey, mint_symbol, vs_token_symbol, quantity, purchase_price) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
            )
            .bind(position.user_pubkey)
            .bind(position.mint_pubkey)
            .bind(position.mint_symbol)
            .bind(position.vs_token_symbol)
            .bind(position.quantity)
            .bind(position.purchase_price)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(position) => Ok(position),
            Err(e) => {
                println!("Error creating position. Error: {}", e);
                Err(ApiError::PositionCreateFail)
            }
        }
    }

    pub async fn get_positions(state: AppState) -> Result<Vec<Self>> {
        println!("->> {:<12} - get_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!("Error fetching positions. Error: {}", e);
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_user_positions(
        user_pubkey: String, 
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_user_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE user_pubkey = $1"
            )
            .bind(&user_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!("Error fetching positions for user: {}. Error: {}", user_pubkey, e);
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_user_positions_by_token(
        user_pubkey: String,
        mint_pubkey: String,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_user_positions_by_token", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE user_pubkey = $1 AND mint_pubkey = $2"
            )
            .bind(&user_pubkey)
            .bind(&mint_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!(
                    "Error fetching positions for user: {}, mint: {}. Error: {}",
                    user_pubkey,
                    mint_pubkey,
                    e
                );
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_token_positions(
        mint_pubkey: String,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_token_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE mint_pubkey = $1"
            )
            .bind(&mint_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!(
                    "Error fetching positions for mint: {}. Error: {}",
                    mint_pubkey,
                    e
                );
                Err(ApiError::PositionGetFail)
            }
        }
    }
}