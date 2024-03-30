use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use crate::{clients::client_jupiter::JupiterClient, errors::Result, models::model_position::{Position, PositionForCreate, PositionWithProfit}, AppState};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/positions", post(create_position).get(get_positions))
        .route("/positions/user/:user_pubkey", get(get_user_positions))
        .route("/positions/user/:user_pubkey/mint/:mint_pubkey", get(get_user_positions_by_token))
        .route("/positions-profit/user/:user_pubkey", get(get_user_positions_and_profit))
        .route("/positions-profit/user/:user_pubkey/mint/:mint_pubkey", get(get_user_positions_and_profit_by_token))
        .route("/positions/mint/:mint_pubkey", get(get_token_positions))
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

async fn get_user_positions(
    State(state): State<AppState>,
    Path(user_pubkey): Path<String>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_user_positions", "HANDLER");

    let positions = Position::get_user_positions(user_pubkey, state).await?;

    Ok(Json(positions))
}

async fn get_user_positions_and_profit(
    State(state): State<AppState>,
    Path(user_pubkey): Path<String>
) -> Result<Json<Vec<PositionWithProfit>>> {
    println!("->> {:<12} - get_user_positions_and_profit", "HANDLER");

    let positions = 
        Position::get_user_positions(user_pubkey, state)
        .await?;

    let mut positions_with_profit: Vec<PositionWithProfit> = Vec::with_capacity(positions.len());

    for position in positions {
        let position_with_profit = JupiterClient::get_position_profit(position).await?;
        positions_with_profit.push(position_with_profit)
    }

    Ok(Json(positions_with_profit))
}

async fn get_user_positions_and_profit_by_token(
    State(state): State<AppState>,
    Path((user_pubkey, mint_pubkey)): Path<(String, String)>
) -> Result<Json<Vec<PositionWithProfit>>> {
    println!("->> {:<12} - get_user_positions_and_profit", "HANDLER");

    let positions = 
        Position::get_user_positions_by_token(user_pubkey, mint_pubkey, state)
        .await?;

    let mut positions_with_profit: Vec<PositionWithProfit> = Vec::with_capacity(positions.len());

    for position in positions {
        let position_with_profit = JupiterClient::get_position_profit(position).await?;
        positions_with_profit.push(position_with_profit)
    }

    Ok(Json(positions_with_profit))
}

async fn get_user_positions_by_token(
    State(state): State<AppState>,
    Path((user_pubkey, mint_pubkey)): Path<(String, String)>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_user_positions_by_token", "HANDLER");

    let positions = Position::get_user_positions_by_token(user_pubkey, mint_pubkey, state).await?;

    Ok(Json(positions))
}

async fn get_token_positions(
    State(state): State<AppState>,
    Path(mint_pubkey): Path<String>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - get_token_positions", "HANDLER");

    let positions = Position::get_token_positions(mint_pubkey, state).await?;

    Ok(Json(positions))
}