use axum_zero2prod::configuration::get_configuration;
use axum_zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind a listener to a port");

    axum::serve(listener, run()).await?;
    Ok(())
}
