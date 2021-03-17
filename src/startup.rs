use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};
use sqlx::PgPool;


pub fn run_actix_backend(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap the pool using web::data, which evals to an Arch smart ptr
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
            App::new()
                // route, HTTP req method, route handler, route match guards 
                .route("/health_check", web::get().to(health_check))
                // TODO add subscriptions GET to enable DB test
                .route("/subscriptions", web::post().to(subscribe))
                // Get a ptr cp and attach/register it to the app state 
                .app_data(db_pool.clone())
        })
        .listen(listener)?
        .run();
    Ok(server)
}

