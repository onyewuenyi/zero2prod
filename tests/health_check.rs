use std::net::TcpListener;
use zero2prod::startup::run_actix_backend;
use zero2prod::configuration::get_configuration;
use sqlx::PgPool;
use uuid::Uuid;
// if there is a type in .header("Content-Type", "application/x-www-form-urlencoded") it will fail with no debug hint

struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors: // if we fail to perform the required setup we can just panic and crash // all the things.
async fn spawn_app() -> TestApp {
    // TODO spin up new DB with rand name
    let config = get_configuration().expect("Failed to read configuration.");
    config.database.database_name = Uuid
    let connection_pool = PgPool::connect(&config.database.conn_str()).await.expect("Failed to connect to Postgres DB.");
    let local_host = "127.0.0.1";
    let listener = TcpListener::bind(format!("{}:0", local_host)).expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", local_host, port);   
    
    
    let server = run_actix_backend(listener, connection_pool.clone()).expect("Failed to listen to port"); // Launch the server as a background task
    // tokio::spawn executes a background process 
    // but we have no use for it here, hence the non-binding let 
    let _ = tokio::spawn(server);
    println!("Spawned app as background ps MF");
    TestApp { address: address, db_pool: connection_pool}
}

 

#[actix_rt::test]
async fn subscribe_returns_200_for_a_valid_form_data() {
    // Return 200 if name and email is valid ans using the application/x-www-form-urlencoded format
    // TODO test for side effects e.g. stored data 
    
    // run server as background ps 
    let app = spawn_app().await;



    // init reqwest obj to send a client http req
    let route = format!("{}/subscriptions", &app.address);
    let client = reqwest::Client::new();

    // define test input data: post method form body data encoded in application/x-www-form-urlencoded"
    // req me to study the data format to encode name and email!!
    let body = "name=charles%20senpai&email=senpai%40gmail.com";
    let res = client
        .post(&route)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute req.");
    
    assert_eq!(200, res.status().as_u16());

    // TODO Update when I changed my testing strategy to BBOX e.g. testing the API and not any internal components that are 
    // coupled to N implementation, which is using X technology e.g. sqlx, postgres 
    let saved = sqlx::query!("SELECT email, name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed ot fetch saved subscription.");
    
        assert_eq!(saved.email, "senpai@gmaill.com");
        assert_eq!(saved.name, "charles senpai");
}

#[actix_rt::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let app = spawn_app().await;
    let address = app.address;
    let route = format!("{}/subscriptions", &address);
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=charles%20senpai", "missing email"),
        ("email=senpai%40gmail.com", "missing name"),
        ("", "missing name and email")
    ];

    for (invalid_body, err_msg) in test_cases {
        // iter a list of things missing states 
        let res = client
            .post(&route)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute req");

        assert_eq!(400, res.status().as_u16(), "The APi did not fail with 400 Bad Request when the payload was {}", err_msg);
    }
}


#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;

    // Send Http req to our app
    let client = reqwest::Client::new();   
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}


