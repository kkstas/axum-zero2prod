use axum::{
    routing::{get, post},
    serve::Serve,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::subscribe;
use crate::{email_client::EmailClient, routes::health_check};

pub struct Application {
    port: u16,
    server: Serve<Router, Router>,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("Invalid sender email address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = tokio::net::TcpListener::bind(address)
            .await
            .expect("Failed to bind a listener to a port");

        let port = listener.local_addr().unwrap().port();
        let server = run(connection_pool, email_client, listener);

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.with_db())
}

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
