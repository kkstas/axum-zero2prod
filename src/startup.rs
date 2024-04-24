use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::routes::health_check;
use crate::routes::subscribe;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn run(db_pool: PgPool) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState { db_pool })
}
