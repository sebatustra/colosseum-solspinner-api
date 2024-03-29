use axum::{extract::State, routing::post, Json, Router};
use crate::{errors::Result, models::model_user::{User, UserForCreate}, AppState};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .with_state(state)
}

async fn create_user(
    State(state): State<AppState>,
    Json(user): Json<UserForCreate>
) -> Result<Json<User>> {
    println!("->> {:<12} - create_user", "HANDLER");

    let user = User::create_user(user, state).await?;

    Ok(Json(user))
}

async fn get_users(
    State(state): State<AppState>
) -> Result<Json<Vec<User>>> {
    println!("->> {:<12} - get_users", "HANDLER");

    let users = User::get_users(state).await?;

    Ok(Json(users))
}