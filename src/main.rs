use zero2prod::run_actix_backend;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    run_actix_backend()?.await
}