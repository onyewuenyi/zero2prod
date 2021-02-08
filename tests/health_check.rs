use std::net::TcpListener;


#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    println!("address = {}", address);

    // Send Http req to our app
    let client = reqwest::Client::new();

   
    let response = client
        .get(&format!("{}/xxxhealth_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors: // if we fail to perform the required setup we can just panic and crash // all the things.
fn spawn_app() -> String {
    // bind 
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();
    
    let server = zero2prod::run_actix_backend(listener).expect("Failed to listen to port"); // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let 
    let _ = tokio::spawn(server);
    println!("Spawned app as background ps MF");
    println!("http://127.0.0.1:{}", port);
    format!("http://127.0.0.1:{}", port)
    
}

 
