use axum::{extract::State, routing::post, Json, Router};
use crate::{
    clients::client_jupiter::JupiterClient, 
    errors::Result, 
    models::model_position::Position, 
    AppState
};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/coordinator/update_prices", 
        post(update_all_positions_prices)
    )
    .with_state(state)
}

pub async fn update_all_positions_prices(
    State(state): State<AppState>
) -> Result<Json<Vec<Position>>> {
    println!("->> {:<12} - update_all_positions_prices", "HANDLER");


    let mints = Position::get_all_mints_in_positions(&state).await?;

    let price_updates = JupiterClient::get_tokens_price(mints).await?;

    let mut results: Vec<Position> = Vec::new();

    for update in price_updates {
        let positions = Position::update_position_price_by_token(update, &state).await?;
        results.extend(positions)
    }

    Ok(Json(results))
}