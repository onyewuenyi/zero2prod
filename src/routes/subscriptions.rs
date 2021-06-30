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




// Create a span at the beginning of the fn invocation
// Attaches all args of the fn to the context of the span 
// implictly get the request_id from TracingLogger in our macro fn annotation
#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, pool)
    fields(
        request_id = %Uuid::new_v4(),
        email = %form.email,
        name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> Result<HttpResponse, HttpResponse>  {
    // Orchestrates the subscribe handler req flow and returns the response to the client 
    // Description: handler for the POST /subscriptions endpoint. It is instrumented with logging and tracing. It takes the req form data (req encoded body) and makes a DB WR req
    // TODO: verify the return: return 200 OK. If name or email is missing return 400 BAD REQUEST

    // Chaos that could occur: db conn down, query is slow do to N, network issues sending req to db in web server e.g. web server goes down

    // Telemetry data emited to stdout: EVENT is a log record and the the rest is the trace span of the entire subscribe flow
    // {"v":0,"name":"test","msg":"[ADDDING A NEW SUBSCRIBER. - START]","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.619794300+00:00","target":"zero2prod::routes::subscriptions","line":25,"file":"src\\routes\\subscriptions.rs","request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - START]","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.620094700+00:00","target":"zero2prod::routes::subscriptions","line":31,"file":"src\\routes\\subscriptions.rs","request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - EVENT] /* SQLx ping */; rows: 0, elapsed: 1.079ms","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.621547400+00:00","target":"log","line":null,"file":null,"log.module_path":"sqlx::query","log.target":"sqlx::query","request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - EVENT] INSERT INTO subscriptions (id, …; rows: 0, elapsed: 3.408ms\n\nINSERT INTO\n  subscriptions (id, email, name, subscribed_as)\nVALUES\n  ($1, $2, $3, $4)\n","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.626969100+00:00","target":"log","line":null,"file":null,"log.target":"sqlx::query","log.module_path":"sqlx::query","request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[SAVING NEW SUBSCRIBER DETAILS IN THE DATABASE - END]","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.627269100+00:00","target":"zero2prod::routes::subscriptions","line":31,"file":"src\\routes\\subscriptions.rs","elapsed_milliseconds":6,"request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[ADDDING A NEW SUBSCRIBER. - EVENT] request_id f0398779-f0ee-4dc9-9b95-6341a6159ac5 - New subscriber details have been saved","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.627667100+00:00","target":"zero2prod::routes::subscriptions","line":53,"file":"src\\routes\\subscriptions.rs","request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    // {"v":0,"name":"test","msg":"[ADDDING A NEW SUBSCRIBER. - END]","level":30,"hostname":"DESKTOP-SSM6UCG","pid":28644,"time":"2021-06-21T13:25:45.627858+00:00","target":"zero2prod::routes::subscriptions","line":25,"file":"src\\routes\\subscriptions.rs","elapsed_milliseconds":7,"request_id":"f0398779-f0ee-4dc9-9b95-6341a6159ac5","email":"senpai@gmail.com"}
    
    insert_subscriber(&pool, &form)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    Ok(HttpResponse::Ok().finish())
}

// Handled by macros now: instrument(tracing::info_span!("<...>")) trace span for an async task 
// attach instrumentation, then `.await` it 
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    // DB logic: try to make a DB WR req with the user email, name, cur date, and a uniq id
    // Decouple from framework app code and abstract DB logic

    // Dependencies: Query use Postgres syntax and Rust SQL toolkit format/calls 
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
    .execute(pool)
    .await
    .map_err(|e| {
        // tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}