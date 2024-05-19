use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::routes::subscribe;
use crate::{email_client::EmailClient, routes::health_check};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub email_client: EmailClient,
}

pub fn run(
    db_pool: PgPool,
    email_client: EmailClient,
    listener: TcpListener,
) -> Serve<Router, Router> {
    let router = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            db_pool,
            email_client,
        });
    axum::serve(listener, router)
}
