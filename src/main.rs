use axum::{middleware, Extension, Router};
use clients::client_birdeye::BirdeyeClient;
use cron_jobs::token_updater::TokenUpdater;
use sqlx::PgPool;
use shuttle_runtime::SecretStore;
use tokio_cron_scheduler::JobScheduler;
use tower_http::cors::CorsLayer;

use crate::cron_jobs::coin_selector::CoinSelector;

mod web;
mod models;
mod errors;
mod clients;
mod utils;
mod cron_jobs;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    api_key: String,
    birdeye_client: BirdeyeClient,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    let _ = sqlx::migrate!().run(&db)
        .await.map_err(|e| format!("Migrations failed. Error: {e}"));

    let api_key = secrets.get("API_KEY")
        .expect("API key not found in secrets!");

    let birdeye_api_key = secrets.get("BIRDEYE_API_KEY")
        .expect("Birdeye API key not found in secrets!");

    let birdeye_client = BirdeyeClient::new(&birdeye_api_key);
    
    let state = AppState { db, api_key, birdeye_client };
    
    let position_routes = web::routes_positions::routes(state.clone());
    let user_routes = web::routes_users::routes(state.clone());
    let token_routes = web::routes_tokens::routes(state.clone());
    let play_routes = web::routes_play::routes(state.clone());

    let api_router = Router::new()
        .merge(position_routes)
        .merge(user_routes)
        .merge(token_routes)
        .merge(play_routes)
        .layer(middleware::from_fn(web::mw_auth::auth_middleware))
        .layer(Extension(state.clone()));
    
    let router = Router::new()
        .nest("/api", api_router)
        .layer(CorsLayer::permissive());

    let scheduler = JobScheduler::new()
        .await.expect("Failed to create job scheduler");

    scheduler.add(
        CoinSelector::init_job("0 */2 * * * *", state.clone())
    ).await.expect("Failed to schedule job");

    scheduler.add(
        TokenUpdater::init_job("0 */3 * * * *", state)
    ).await.expect("Failed to schedule job");

    scheduler.start().await.expect("Failed to start scheduler");

    Ok(router.into())
}