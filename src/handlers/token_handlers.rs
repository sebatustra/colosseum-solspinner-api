use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};

use crate::{
    models::token::{
        Token, 
        TokenCreation
    }, 
    AppState
};

pub async fn create_token(
    State(state): State<AppState>,
    Json(body): Json<TokenCreation>
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
            "INSERT INTO tokens (mint_public_key, symbol, name) VALUES ($1, $2, $3)"
        )
        .bind(body.mint_public_key)
        .bind(body.symbol)
        .bind(body.name)
        .execute(&state.db)
        .await {
            return Err(
                (StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while inserting token: {e}"))
            );
        };

    Ok(StatusCode::OK)
}

pub async fn fetch_all_tokens(
    State(state): State<AppState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, Token>(
        "SELECT * FROM tokens"
        )
        .fetch_all(&state.db).await {
            Ok(res) => res,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
            }
        };

    Ok(Json(res))
}