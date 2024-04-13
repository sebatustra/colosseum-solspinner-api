use axum::{extract::State, routing::get, Json, Router};

use crate::{errors::api_errors::Result, models::model_token::{Token, TokenForClient}, AppState};


pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/play/coins", get(get_all_active_tokens))
        .route("/play/coins-filtered", get(get_7_active_selected_tokens))
        .route("/play/run", get(get_random_token))
        .with_state(state)
}

async fn get_all_active_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<TokenForClient>>> {
    println!("->> {:<12} - get_all_active_tokens", "HANDLER");

    let tokens = Token::get_all_active_tokens(state).await?;

    Ok(Json(tokens))
}

async fn get_7_active_selected_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<TokenForClient>>> {
    println!("->> {:<12} - get_7_active_selected_tokens", "HANDLER");

    let tokens = Token::get_7_active_tokens(state).await?;

    Ok(Json(tokens))
}

async fn get_random_token(
    State(state): State<AppState>
) -> Result<Json<TokenForClient>> {
    println!("->> {:<12} - get_random_token", "HANDLER");

    let tokens = Token::get_all_active_tokens(state).await?;

    if tokens.is_empty() {
        return Err(crate::errors::api_errors::ApiError::SelectedTokenGetFail)
    }
    
    let random_index = rand::random::<usize>() % tokens.len();
    
    let randomly_selected_token = tokens[random_index].clone();

    Ok(Json(randomly_selected_token))
}
