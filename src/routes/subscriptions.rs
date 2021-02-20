
use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // get name and email from the POST /subscribers req using application/x-www-fornm-urlencoded
    // parse req encoded body of a POST REQ
    // return 200 OK
    // if name or email is missing return 400 BAD REQUEST
    // conn to DB
    
    // extract data from encoded body using actix extractor 
    println!("Welcome {}, {}!", form.name, form.email);
    HttpResponse::Ok().finish()
}
