use axum::{middleware, Extension, Router};
use sqlx::PgPool;
use shuttle_runtime::SecretStore;


mod web;
mod models;
mod errors;
mod clients;
mod utils;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    api_key: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    // db.execute(include_str!("../migrations.sql")).await.unwrap();

    let _ = sqlx::migrate!().run(&db).await.map_err(|e| format!("Migrations failed. Error: {e}"));

    let api_key = secrets.get("API_KEY").expect("API key not found in secrets!");

    let state = AppState { db, api_key };

    let position_routes = web::routes_positions::routes(state.clone());
    let user_routes = web::routes_users::routes(state.clone());
    let token_routes = web::routes_tokens::routes(state.clone());

    let api_router = Router::new()
        .merge(position_routes)
        .merge(user_routes)
        .merge(token_routes)
        .layer(middleware::from_fn(web::mw_auth::auth_middleware))
        .layer(Extension(state.clone()));
    
    let router = Router::new().nest("/api", api_router);

    Ok(router.into())
}
