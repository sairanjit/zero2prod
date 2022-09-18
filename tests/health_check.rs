#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    // Create a client
    let client = reqwest::Client::new();

    // ACT
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Asert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
