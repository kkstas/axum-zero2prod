use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::routes::health_check;
use crate::routes::subscribe;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn run(db_pool: PgPool) -> Router {
    let state = AppState { db_pool };
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
}
