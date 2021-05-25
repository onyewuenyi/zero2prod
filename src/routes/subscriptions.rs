
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing_futures::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse>  {
    // Choas that could occur: db conn down, query is slow do to N, network issues sending req to db in web server e.g. web server goes down

    // get name and email from the POST /subscribers req using application/x-www-fornm-urlencoded
    // parse req encoded body of a POST REQ
    // return 200 OK
    // if name or email is missing return 400 BAD REQUEST
    // conn to DB
    
    let req_id = Uuid::new_v4();
    // create and step into span 
    let req_span = tracing::info_span!("Addding a new subscriber.", %req_id, email = %form.email, name = %form.name);
    let _req_span_guard = req_span.enter();

    // tracing::info!("request_id {} - Database: {:?}", req_id, pool.get_ref());
    // tracing::info!("request_id {} - Saving new subscriber in the database", req_id);

    let query_span = tracing::info_span!("Saving new subscriber details in the database");
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_as)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // TODO refractor later - Rust lang
    // conn = Arc<Arc<PgConnection>>
    // conn.get_reg() = &Arc<PgConnection>
    // <>.deref = &PgConnection
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    .map_err(|e| {
        tracing::error!("request_id {} - Failed to execute query: {:?}", req_id, e);
        HttpResponse::InternalServerError().finish()
    })?;
    tracing::info!("request_id {} - New subscriber details have been saved", req_id);
    Ok(HttpResponse::Ok().finish())
}
