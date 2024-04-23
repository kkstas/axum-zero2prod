#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:5050")
        .await
        .expect("Failed to bind a listener to a port");

    axum::serve(listener, axum_zero2prod::run()).await.unwrap();
}
