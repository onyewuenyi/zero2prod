

use actix_web::{HttpResponse};

// #[derive(serde::Deserialize)]
// TODO mv API req handlers to a mod 
// GET /health_check endpoint handler 

pub async fn health_check() -> HttpResponse {
    // Ok() =  (200, OK, "OK")
    // finish(): Set an empty body and generate Response
    HttpResponse::Ok().finish()
}
