use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};

use crate::{
    models::user::{
        User, 
        UserCreation
    }, 
    AppState
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<UserCreation>
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
            "INSERT INTO users (public_key) VALUES ($1)"
        )
        .bind(body.public_key)
        .execute(&state.db)
        .await {
            return Err(
                (StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while inserting user: {e}"))
            );
        };

    Ok(StatusCode::OK)
}

pub async fn fetch_all_users(
    State(state): State<AppState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, User>(
        "SELECT * FROM users"
        )
        .fetch_all(&state.db).await {
            Ok(res) => res,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            }
        };

    Ok(Json(res))
}