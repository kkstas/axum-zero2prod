use axum::{
    routing::{get, post},
    Router,
};

use crate::health_check;
use crate::subscribe;

pub fn run() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}
