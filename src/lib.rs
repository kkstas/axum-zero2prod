use axum::{http::StatusCode, routing::get, Router};

pub fn run() -> Router {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
