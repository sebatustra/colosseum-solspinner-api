use serde::Deserialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::{clients::clients_structs::TokenFromClient, AppState, errors::{ApiError, Result}};

#[derive(Deserialize, Debug, FromRow)]
pub struct SelectedToken {
    pub id: Uuid,
    pub mint_pubkey: String,
    pub symbol: String,
    pub name: String,
    pub logo_url: String,
    pub price_change_24h_percent: f64,
    pub volume_24h_usd: f64,
    pub created_at: chrono::DateTime<chrono::Utc>
}

impl SelectedToken {
    pub async fn create_selected_token(
        token: TokenFromClient,
        state: AppState
    ) -> Result<()> {
        println!("->> {:<12} - create_selected_token", "CONTROLLER");

        todo!()
    }
}
