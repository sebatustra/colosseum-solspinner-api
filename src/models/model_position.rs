use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::{errors::api_errors::{ApiError, Result}, AppState};

#[derive(sqlx::FromRow, Debug, Serialize, Clone)]
pub struct Position {
    pub id: Uuid,
    pub user_pubkey: String,
    pub token_pubkey: String,
    pub token_symbol: String,
    pub token_logo_url: String,
    pub vs_token_pubkey: String,
    pub vs_token_symbol: String,
    pub vs_token_logo_url: String,
    pub initial_quantity: f64,
    pub current_quantity: f64,
    pub purchase_price: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct PositionWithProfit {
    pub id: Uuid,
    pub user_pubkey: String,
    pub token_pubkey: String,
    pub token_symbol: String,
    pub token_logo_url: String,
    pub vs_token_pubkey: String,
    pub vs_token_symbol: String,
    pub vs_token_logo_url: String,
    pub initial_quantity: f64,
    pub current_quantity: f64,
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
            token_pubkey: position.token_pubkey,
            token_symbol: position.token_symbol,
            token_logo_url: position.token_logo_url,
            vs_token_pubkey: position.vs_token_pubkey,
            vs_token_symbol: position.vs_token_symbol,
            vs_token_logo_url: position.vs_token_logo_url,
            initial_quantity: position.initial_quantity,
            current_quantity: position.current_quantity,
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
    pub token_pubkey: String,
    pub token_symbol: String,
    pub token_logo_url: String,
    pub vs_token_pubkey: String,
    pub vs_token_symbol: String,
    pub vs_token_logo_url: String,
    pub quantity: f64,
    pub purchase_price: f64,
}

#[derive(Deserialize, Debug, sqlx::FromRow)]
pub struct UniquePositionsData {
    pub token_pubkey: String,
    pub vs_token_symbol: String
}

#[derive(Deserialize, Debug)]
pub struct UpdatePositionData {
    pub position_id: Uuid,
    pub new_quantity: f64
}

// CRUD implementation for Position

impl Position {
    pub async fn create_position(
        position: PositionForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_position", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "INSERT INTO positions (user_pubkey, token_pubkey, token_symbol, token_logo_url, vs_token_pubkey, vs_token_symbol, vs_token_logo_url, initial_quantity, current_quantity, purchase_price) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *;"
            )
            .bind(position.user_pubkey)
            .bind(position.token_pubkey)
            .bind(position.token_symbol)
            .bind(position.token_logo_url)
            .bind(position.vs_token_pubkey)
            .bind(position.vs_token_symbol)
            .bind(position.vs_token_logo_url)
            .bind(position.quantity)
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

    pub async fn update_position_quantity(
        update_data: UpdatePositionData,
        state: AppState
    ) -> Result<Position> {
        println!("->> {:<12} - update_position", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "UPDATE positions SET current_quantity = $1 WHERE id = $2 RETURNING *"
            )
            .bind(update_data.new_quantity)
            .bind(&update_data.position_id)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(position) => Ok(position),
            Err(e) => {
                println!("Error updating position with id: {}. Error: {}",update_data.position_id, e);
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_positions(state: AppState) -> Result<Vec<Self>> {
        println!("->> {:<12} - get_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions;"
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
        user_pubkey: &str, 
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_user_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE user_pubkey = $1;"
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
        user_pubkey: &str,
        token_pubkey: &str,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_user_positions_by_token", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE user_pubkey = $1 AND token_pubkey = $2;"
            )
            .bind(&user_pubkey)
            .bind(&token_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!(
                    "Error fetching positions for user: {}, mint: {}. Error: {}",
                    user_pubkey,
                    token_pubkey,
                    e
                );
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_token_positions(
        token_pubkey: &str,
        state: AppState
    ) -> Result<Vec<Position>> {
        println!("->> {:<12} - get_token_positions", "CONTROLLER");

        let result = sqlx::query_as::<_, Position>(
                "SELECT * FROM positions WHERE token_pubkey = $1;"
            )
            .bind(&token_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(positions) => Ok(positions),
            Err(e) => {
                println!(
                    "Error fetching positions for mint: {}. Error: {}",
                    token_pubkey,
                    e
                );
                Err(ApiError::PositionGetFail)
            }
        }
    }

    pub async fn get_user_unique_tokens_and_vs_tokens(
        user_pubkey: &str,
        state: AppState
    ) -> Result<Vec<UniquePositionsData>> {
        println!("->> {:<12} - get_unique_positions_pubkey_and_vs_token", "CONTROLLER");

        let result = sqlx::query_as::<_, UniquePositionsData>(
                "SELECT DISTINCT ON (token_pubkey, vs_token_symbol) token_pubkey, vs_token_symbol FROM positions WHERE user_pubkey = $1 ORDER BY token_pubkey, vs_token_symbol, created_at;"
            )
            .bind(user_pubkey)
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(unique_positions) => Ok(unique_positions),
            Err(e) => {
                println!(
                    "Error fetching unique positions for user: {}. Error: {}",
                    user_pubkey,
                    e
                );
                Err(ApiError::PositionGetFail)
            }
        }
    }
}