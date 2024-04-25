use axum_zero2prod::configuration::get_configuration;
use axum_zero2prod::startup::run;
use axum_zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("axum-zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let db_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind a listener to a port");

    axum::serve(listener, run(db_pool)).await?;
    Ok(())
}
