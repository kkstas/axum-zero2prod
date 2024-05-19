use axum_zero2prod::configuration::get_configuration;
use axum_zero2prod::email_client::EmailClient;
use axum_zero2prod::startup::run;
use axum_zero2prod::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("axum-zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
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
    let db_pool = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind a listener to a port");

    run(db_pool, email_client, listener).await?;
    Ok(())
}
