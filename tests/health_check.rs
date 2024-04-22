#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:9090/health_check")
        .send()
        .await
        .expect("Failed to execute a request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9090")
        .await
        .unwrap();
    let _ = tokio::spawn(async move {
        axum::serve(listener, axum_z2p::run()).await.unwrap();
    });
}
