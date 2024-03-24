use axum::{routing::{get, post}, Router};
use sqlx::{PgPool, Executor};

pub mod handlers;
pub mod models;

use handlers::{
    user_handlers::{
        fetch_all_users,
        create_user
    },
    token_handlers::{
        fetch_all_tokens,
        create_token
    },
    position_handlers::{
        fetch_all_positions,
        create_position
    }
};

#[derive(Clone)]
pub struct AppState {
    db: PgPool
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> shuttle_axum::ShuttleAxum {

    db.execute(include_str!("../migrations.sql")).await.unwrap();

    let state = AppState { db };

    let router = Router::new()
        .route("/users", get(fetch_all_users))
        .route("/users", post(create_user))
        .route("/tokens", get(fetch_all_tokens))
        .route("/tokens", post(create_token))
        .route("/positions", get(fetch_all_positions))
        .route("/positions", post(create_position))
        .with_state(state);

    Ok(router.into())
}
