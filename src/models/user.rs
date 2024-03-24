use serde::{Deserialize, Serialize};


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub public_key: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct UserCreation {
    pub public_key: String,
}
