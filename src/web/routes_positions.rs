use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use crate::{models::model_position::{PositionForCreate, Position}, AppState, errors::Result};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/positions", post(create_position).get(get_positions))
        .route("/positions/user/:user_pubkey", get(get_all_user_positions))
        .route("/positions/user/:user_pubkey/mint/:mint_pubkey", get(get_all_user_positions_by_token))
        .route("/positions/mint/:mint_pubkey", get(get_all_token_positions))
        .with_state(state)
}

async fn create_position(
    State(state): State<AppState>,
    Json(position): Json<PositionForCreate>
) -> Result<Json<Position>> {
    println!("->> {:<12} - create_position", "HANDLER");

    let position = Position::create_position(position, state).await?;

    Ok(Json(position))
}

async fn get_positions(
    State(state): State<AppState>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_positions", "HANDLER");

    let positions = Position::get_positions(state).await?;

    Ok(Json(positions))
}

async fn get_all_user_positions(
    State(state): State<AppState>,
    Path(user_pubkey): Path<String>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_user_positions", "HANDLER");

    let positions = Position::get_all_user_positions(user_pubkey, state).await?;

    Ok(Json(positions))
}

async fn get_all_user_positions_by_token(
    State(state): State<AppState>,
    Path((user_pubkey, mint_pubkey)): Path<(String, String)>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_user_positions_by_token", "HANDLER");

    let positions = Position::get_all_user_positions_by_token(user_pubkey, mint_pubkey, state).await?;

    Ok(Json(positions))
}

async fn get_all_token_positions(
    State(state): State<AppState>,
    Path(mint_pubkey): Path<String>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_token_positions", "HANDLER");

    let positions = Position::get_all_token_positions(mint_pubkey, state).await?;

    Ok(Json(positions))
}