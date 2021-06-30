use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;
use tracing::Subscriber;


// env_logger == (feature-parity) tracing_subscriber::filter::EnvLogger, 
// tracing_buyan_formatter::JsonStorageLayer, tracing_buyan_formatter::BunyanFormatterLayer


/// Compose multiple layers into a `tracing`'s subscriber. ///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to /// spell out the actual type of the returned subscriber, which is /// indeed quite complex.
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on.
pub fn get_subscriber(name: String, env_filter: String) -> impl Subscriber + Send + Sync {
    // Relative to tracing log, log does not emit tracing events out of the box and does not provide a feature flag to enable relative to tracing
    // Must explictly register a logger imp 
    // Redirect all log's events to our subscriber 

    // Print all spans at info level or above by default if RUST_LOG env var has not been set 
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(env_filter));
    
    // Format the span the emitted info and print to stdout 
    let formatting_layer = BunyanFormattingLayer::new(name, std::io::stdout);
    
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}


/// Register a subscriber as global default to process span data. 
/// 
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    //  solves the prob of emitting actix-web log records by enabling log to emit tracing events to log's logger 
    // we explictly register a logger imp to redirect the logs to our tracing subscriber for processing 
    // this results in a failure when integration test calls this N times for each unit test.
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}