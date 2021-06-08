use zero2prod::startup::run_actix_backend;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
mod routes;




#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let subcriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subcriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.conn_str())
        .await
        .expect("Failed to connect to Postgres");
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.app_port))?;
    
    // . await will listen to the address indefintly 
    run_actix_backend(listener, connection_pool)?.await
}

