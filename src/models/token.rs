use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct Token {
    pub mint_public_key: String,
    pub symbol: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct TokenCreation {
    pub mint_public_key: String,
    pub symbol: String,
    pub name: String,
}