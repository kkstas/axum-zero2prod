use axum_zero2prod::configuration::{get_configuration, DatabaseSettings};
use axum_zero2prod::domain::SubscriberEmail;
use axum_zero2prod::email_client::EmailClient;
use axum_zero2prod::startup::run;
use axum_zero2prod::telemetry::{get_subscriber, init_subscriber};
use fake::{Fake, Faker};
use once_cell::sync::Lazy;
use secrecy::Secret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(&configuration.database).await;
    let spawn_pool = db_pool.clone();

    let _ = tokio::spawn(async move {
        run(
            spawn_pool,
            EmailClient::new(
                Faker.fake(),
                SubscriberEmail::parse("mailtrap@demomailtrap.com".to_string()).unwrap(),
                Secret::new(Faker.fake()),
                std::time::Duration::from_millis(200),
            ),
            listener,
        )
        .await
    });
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
