use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use crate::{clients::client_jupiter::PriceUpdate, errors::{ApiError, Result}, AppState};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Position {
    pub id: Uuid,
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub current_price: f64,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Debug)]
pub struct PositionForCreate {
    pub user_pubkey: String,
    pub mint_pubkey: String,
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

        let now = Utc::now();

        let result = sqlx::query_as::<_, Position>(
                "INSERT INTO positions (user_pubkey, mint_pubkey, quantity, purchase_price, current_price, purchase_date, last_updated) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
            )
            .bind(position.user_pubkey)
            .bind(position.mint_pubkey)
            .bind(position.quantity)
            .bind(position.purchase_price)
            .bind(position.purchase_price)
            .bind(now)
            .bind(now)
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

    pub async fn get_all_user_positions(
        user_pubkey: String, 
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_all_user_positions", "CONTROLLER");

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

    pub async fn get_all_user_positions_by_token(
        user_pubkey: String,
        mint_pubkey: String,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_all_user_positions_by_token", "CONTROLLER");

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

    pub async fn get_all_token_positions(
        mint_pubkey: String,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_all_token_positions", "CONTROLLER");

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

    pub async fn get_all_mints_in_positions(
        state: &AppState
    ) -> Result<Vec<String>> {
        println!("->> {:<12} - get_all_mints_in_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, PositionMint>(
                "SELECT DISTINCT mint_pubkey FROM positions"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(mints) => Ok(mints.into_iter().map(|mint| mint.mint_pubkey).collect()),
            Err(e) => {
                println!(
                    "Error fetching distinct mints. Error: {}",
                    e
                );

                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn update_position_price_by_token(
        price_update: PriceUpdate,
        state: &AppState
    ) -> Result<Vec<Position>>{
        println!("->> {:<12} - update_position_price_by_token", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "UPDATE positions SET current_price = $1, last_updated = $2 WHERE mint_pubkey = $3 RETURNING *"
            )
            .bind(price_update.new_price)
            .bind(Utc::now())
            .bind(&price_update.mint_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!(
                    "Error updating positions for mint: {}. Error: {}",
                    price_update.mint_pubkey,
                    e
                );
                Err(ApiError::PositionUpdateFail)
            }
        }
    }
}