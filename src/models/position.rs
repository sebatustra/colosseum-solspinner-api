use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub current_value: f64,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct PositionCreation {
    pub user_pubkey: String,
    pub mint_pubkey: String,
    pub quantity: f64,
    pub purchase_price: f64,
    pub current_value: f64,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
}