use zero2prod::startup::run_actix_backend;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.conn_str())
        .await
        .expect("Failed to connect to Postgres");
    
    println!("LOG: {}", configuration.database.conn_str());

    let address = format!("127.0.0.1:{}", configuration.app_port);
    println!("LOG: <LocalHost>:<Port>: {}", address);

    let listener = TcpListener::bind(address)?;
    run_actix_backend(listener, connection_pool)?.await
}

