use axum::{extract::State, routing::get, Json, Router};
use crate::{models::model_token::Token, AppState, errors::api_errors::Result};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/tokens", get(get_tokens))
        .with_state(state)
}

async fn get_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<Token>>> {
    println!("->> {:<12} - get_tokens", "HANDLER");

    let tokens = Token::get_tokens(state).await?;

    Ok(Json(tokens))
}

