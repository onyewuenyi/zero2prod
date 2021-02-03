#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    // Send Http req to our app
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3030/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run_actix_backend().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}