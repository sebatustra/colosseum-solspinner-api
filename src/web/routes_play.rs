use axum::{extract::State, routing::get, Json, Router};

use crate::{errors::Result, models::model_selected_token::SelectedToken, AppState};


pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/play/coins", get(get_all_active_selected_tokens))
        .route("/play/coins-filtered", get(get_7_active_selected_tokens))
        .route("/play/run", get(get_random_selected_token))
        .with_state(state)
}

async fn get_all_active_selected_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<SelectedToken>>> {
    println!("->> {:<12} - get_all_active_selected_tokens", "HANDLER");

    let tokens = SelectedToken::get_all_active_selected_tokens(state).await?;

    Ok(Json(tokens))
}

async fn get_7_active_selected_tokens(
    State(state): State<AppState>
) -> Result<Json<Vec<SelectedToken>>> {
    println!("->> {:<12} - get_7_active_selected_tokens", "HANDLER");

    let tokens = SelectedToken::get_7_active_selected_tokens(state).await?;

    Ok(Json(tokens))
}

async fn get_random_selected_token(
    State(state): State<AppState>
) -> Result<Json<SelectedToken>> {
    println!("->> {:<12} - get_random_selected_token", "HANDLER");

    let tokens = SelectedToken::get_all_active_selected_tokens(state).await?;

    if tokens.is_empty() {
        return Err(crate::errors::ApiError::SelectedTokenGetFail)
    }
    
    let random_index = rand::random::<usize>() % tokens.len();
    
    let randomly_selected_token = tokens[random_index].clone();

    Ok(Json(randomly_selected_token))
}
