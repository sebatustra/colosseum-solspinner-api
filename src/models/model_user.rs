use serde::{Deserialize, Serialize};
use crate::{errors::{ApiError, Result}, AppState};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub user_pubkey: String,
    pub created_at: chrono::DateTime<chrono::Utc>
}

#[derive(Debug, Deserialize)]
pub struct UserForCreate {
    pub user_pubkey: String,
}

// CRUD implementation for User

impl User {
    pub async fn create_user(
        user: UserForCreate, 
        state: AppState
    ) -> Result<Self> {
        println!("->> {:<12} - create_user", "CONTROLLER");

        let result = sqlx::query_as::<_, User>(
                "INSERT INTO users (user_pubkey) VALUES ($1) RETURNING *"
            )
            .bind(user.user_pubkey)
            .fetch_one(&state.db)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                println!("Error creating user. Error: {}", e);
                Err(ApiError::UserCreateFail)
            }
        }
    }

    pub async fn get_users(state: AppState) -> Result<Vec<Self>> {
        println!("->> {:<12} - get_users", "CONTROLLER");

        let result = sqlx::query_as::<_, User>(
                "SELECT * FROM users"
            )
            .fetch_all(&state.db)
            .await;

        match result {
            Ok(users) => Ok(users),
            Err(e) => {
                println!("Error fetching users. Error: {}", e);
                Err(ApiError::UserGetFail)
            }
        }
    }

    pub async fn get_user(
        pubkey: String, 
        state: AppState
    ) -> Result<Option<User>> {
        println!("->> {:<12} - get_user", "CONTROLLER");

        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE user_pubkey = $1"
        )
        .bind(&pubkey)
        .fetch_optional(&state.db)
        .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => {
                println!("Error fetching user with pubkey {}. Error: {}", pubkey, e);
                Err(ApiError::UserGetFail)
            }
        }
    }
}