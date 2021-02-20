use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};


pub fn run_actix_backend(listener: TcpListener) -> Result<Server, std::io::Error> {
    // Setup Postgres DB: Rust driver
    let server = HttpServer::new(|| {
            App::new()
                // route, HTTP req method, route handler, route match guards 
                .route("/health_check", web::get().to(health_check))
                .route("/subscriptions", web::post().to(subscribe))
        })
        .listen(listener)?
        .run();
    Ok(server)
}
