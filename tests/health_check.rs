use std::net::TcpListener;

#[actix_rt::test]
async fn health_check_works() {
    // Get address
    let address = spawn_app();

    // Create a client
    let client = reqwest::Client::new();

    // ACT
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Asert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to find random port");

    // Get a random port from OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
