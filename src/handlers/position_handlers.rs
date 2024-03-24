use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};

use crate::{
    models::position::{
        Position, 
        PositionCreation
    }, 
    AppState
};

pub async fn create_position(
    State(state): State<AppState>,
    Json(body): Json<PositionCreation>
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = sqlx::query(
            "INSERT INTO positions (user_pubkey, mint_pubkey, quantity, purchase_price, current_value, purchase_date) VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(body.user_pubkey)
        .bind(body.mint_pubkey)
        .bind(body.quantity)
        .bind(body.purchase_price)
        .bind(body.current_value)
        .bind(body.purchase_date)
        .execute(&state.db)
        .await {
            return Err(
                (StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error while inserting position: {e}"))
            );
        };

    Ok(StatusCode::OK)
}

pub async fn fetch_all_positions(
    State(state): State<AppState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let res = match sqlx::query_as::<_, Position>(
            "SELECT * FROM positions"
        )
        .fetch_all(&state.db).await {
            Ok(res) => res,
            Err(e) => {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
            }
        };

    Ok(Json(res))
}