use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use std::net::TcpListener;


// #[derive(serde::Deserialize)]
// TODO mv API req handlers to a mod 
// GET /health_check endpoint handler 
async fn health_check() -> HttpResponse {
    // Ok() =  (200, OK, "OK")
    // finish(): Set an empty body and generate Response
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // get name and email from the POST /subscribers req using application/x-www-fornm-urlencoded
    // parse req encoded body of a POST REQ
    // return 200 OK
    // if name or email is missing return 400 BAD REQUEST
    // conn to DB
    
    // extract data from encoded body using actix extractor 
    println!("Welcome {}, {}!", form.name, form.email);
    
    HttpResponse::Ok().finish()
}

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


