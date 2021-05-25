use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};
use sqlx::PgPool;



pub fn run_actix_backend(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::data, which evals to an Arch smart ptr; req for how actix works 
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
            App::new()
                // Logger prints to stdout a log record for every incoming req 
                // add middlewares in actix using wrap
                .wrap(Logger::default())
                // route, HTTP req method, route handler, route match guards 
                .route("/health_check", web::get().to(health_check))
                // TODO add subscriptions GET to enable DB test
                .route("/subscriptions", web::post().to(subscribe))
                // Get a ptr cp and attach/register it to the app state 
                .app_data(connection_pool.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}

