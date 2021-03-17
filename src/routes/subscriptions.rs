
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse>  {
    // get name and email from the POST /subscribers req using application/x-www-fornm-urlencoded
    // parse req encoded body of a POST REQ
    // return 200 OK
    // if name or email is missing return 400 BAD REQUEST
    // conn to DB
    
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
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    Ok(HttpResponse::Ok().finish())
}
