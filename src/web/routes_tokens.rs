use axum::{extract::State, routing::post, Json, Router};
use crate::{models::model_token::{Token, TokenForCreate}, AppState, errors::Result};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/tokens", post(create_token).get(get_tokens))
        .with_state(state)
}

async fn create_token(
    State(state): State<AppState>,
    Json(token): Json<TokenForCreate>
) -> Result<Json<Token>> {
    println!("->> {:<12} - create_token", "HANDLER");

    let token = Token::create_token(token, state).await?;

    Ok(Json(token))
}

async fn get_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<Token>>> {
    println!("->> {:<12} - get_tokens", "HANDLER");

    let tokens = Token::get_tokens(state).await?;

    Ok(Json(tokens))
}

