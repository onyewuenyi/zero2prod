use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;


// GET /health_check endpoint handler 
async fn health_check() -> HttpResponse {
    // Ok() =  (200, OK, "OK")
    // finish(): Set an empty body and generate Response
    HttpResponse::Ok().finish()
}

pub fn run_actix_backend(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
            App::new()
                .route("/health_check", web::get().to(health_check))
        })
        .listen(listener)?
        .run();
    Ok(server)
}


