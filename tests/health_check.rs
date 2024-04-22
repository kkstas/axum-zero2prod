#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute a request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let _ = tokio::spawn(async move {
        axum::serve(listener, axum_z2p::run()).await.unwrap();
    });
    format!("http://127.0.0.1:{}", port)
}
