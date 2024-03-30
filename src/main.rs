use axum::Router;
use sqlx::{PgPool, Executor};

mod web;
mod models;
mod errors;
mod clients;

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

    let position_routes = web::routes_positions::routes(state.clone());
    let user_routes = web::routes_users::routes(state.clone());
    let token_routes = web::routes_tokens::routes(state.clone());

    let api_router = Router::new()
        .merge(position_routes)
        .merge(user_routes)
        .merge(token_routes);
    
    let router = Router::new().nest("/api", api_router);

    Ok(router.into())
}
