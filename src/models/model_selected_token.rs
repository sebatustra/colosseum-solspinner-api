use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{clients::clients_structs::TokenFromClient, AppState, errors::{ApiError, Result}};

#[derive(Deserialize, Debug, sqlx::FromRow, Clone, Serialize)]
pub struct SelectedToken {
    pub id: Uuid,
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub price_change_24h_percent: f64,
    pub volume_24h_usd: f64,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>
}

impl SelectedToken {
    pub async fn create_selected_token(
        token: TokenFromClient,
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_selected_token", "CONTROLLER");

        let result = sqlx::query_as::<_, SelectedToken>(
                "INSERT INTO selected_tokens (mint_pubkey, symbol, name, logo_url, price_change_24h_percent, volume_24h_usd) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
            )
            .bind(token.address)
            .bind(token.symbol)
            .bind(token.name)
            .bind(token.logo_uri)
            .bind(token.price_change_24h_percent)
            .bind(token.volume_24h_usd)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(token) => Ok(token),
            Err(e) => {
                println!("Error creating selected token. Error: {}", e);
                Err(ApiError::SelectedTokenCreateFail)
            }
        }
    }

    pub async fn get_all_active_selected_tokens(
        state: AppState
    ) -> Result<Vec<SelectedToken>> {
        println!("->> {:<12} - get_all_active_selected_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, SelectedToken>(
                "SELECT * FROM selected_tokens WHERE is_active = true"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                println!("Error fetching tokens. Error: {}", e);
                Err(ApiError::SelectedTokenGetFail)
            }
        }
    }

    pub async fn get_7_active_selected_tokens(
        state: AppState
    ) -> Result<Vec<SelectedToken>> {
        println!("->> {:<12} - get_7_active_selected_tokens", "CONTROLLER");

        let result = sqlx::query_as::<_, SelectedToken>(
                "SELECT * FROM selected_tokens WHERE is_active = true ORDER BY volume_24h_usd DESC LIMIT 7"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(tokens) => Ok(tokens),
            Err(e) => {
                println!("Error fetching tokens. Error: {}", e);
                Err(ApiError::SelectedTokenGetFail)
            }
        }
    }

    pub async fn update_actives_to_inactive(
        state: AppState
    ) -> Result<()> {
        println!("->> {:<12} - update_to_inactive", "CONTROLLER");

        let result = sqlx::query(
                "UPDATE selected_tokens SET is_active = false WHERE is_active = true"
            )
            .execute(&state.db)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Error updating tokens to inactive. Error: {}", e);
                Err(ApiError::SelectedTokenUpdateFail)  // Assuming you have this variant in your ApiError enum
            }
        }
    }
}
