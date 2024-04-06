use axum::{middleware, Extension, Router};
use sqlx::PgPool;
use shuttle_runtime::SecretStore;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::coin_selector::CoinSelector;

mod web;
mod models;
mod errors;
mod clients;
mod utils;
mod coin_selector;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
    api_key: String,
    birdeye_api_key: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {

    let _ = sqlx::migrate!().run(&db).await.map_err(|e| format!("Migrations failed. Error: {e}"));

    let api_key = secrets.get("API_KEY").expect("API key not found in secrets!");

    let birdeye_api_key = secrets.get("BIRDEYE_API_KEY").expect("Birdeye API key not found in secrets!");

    let state = AppState { db, api_key, birdeye_api_key };

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

    let scheduler = JobScheduler::new().await.expect("Failed to create job scheduler");

    scheduler.add(
        Job::new_async("1 * * * * *", move |_, _| {
            let state_copy = state.clone();
            Box::pin(async move {
                CoinSelector::run_coin_selection(state_copy.clone()).await;
                println!("i run async every 10 seconds");
            })
        }).expect("Failed to add job")
    ).await.expect("Failed to schedule job");

    scheduler.start().await.expect("Failed to start scheduler");

    Ok(router.into())
}