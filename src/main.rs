use zero2prod::startup::run_actix_backend;
use zero2prod::configuration::get_configuration;
use std::net::TcpListener;
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Relative to tracing log, log does not emit tracing events out of the box and does not provide a feature flag to enable relative to tracing
    // Must explictly register a logger imp 
    // Redirect all log's events to our subscriber 
    LogTracer::init().expect("Failed to set logger");

    // Print all spans at info level or above by default if RUST_LOG env var has not been set 
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    // Format the span the emitted info and print to stdout 
    let formatting_layer = BunyanFormattingLayer::new("zero2prod".into(), std::io::stdout);
    
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.conn_str())
        .await
        .expect("Failed to connect to Postgres");
    
    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.app_port))?;
    
    // . await will listen to the address indefintly 
    run_actix_backend(listener, connection_pool)?.await
}

