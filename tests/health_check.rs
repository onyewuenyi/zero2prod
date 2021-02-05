#[actix_rt::test]
async fn health_check_works() {
    spawn_app();

    // Send Http req to our app
    let client = reqwest::Client::new();

    let route = "http://127.0.0.1:8000/health_check";
    let response = client
        .get(route)
        .send()
        .await
        .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors: // if we fail to perform the required setup we can just panic and crash // all the things.
fn spawn_app() {
    
    // New dev dependency - let's add tokio to the party with
    // `cargo add tokio --dev --vers 0.2.22`
    let server = zero2prod::run_actix_backend().expect("Failed to bind address"); // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let 
    let _ = tokio::spawn(server);
    println!("Spawned app as background ps MF");
}


// ---- health_check_works stdout ----
// Spawned app as background ps MF
// thread 'health_check_works' panicked at 'Failed to execute request.: 
// reqwest::Error { kind: Request, url: Url { scheme: "http", host: Some(Ipv4(127.0.0.1)), 
// port: Some(3030), path: "/health_check", query: None, fragment: None }, 
// source: hyper::Error(Connect, ConnectError("tcp connect error", 
// Os { code: 61, kind: ConnectionRefused, message: "Connection refused" })) }', 
// tests/health_check.rs:13:10