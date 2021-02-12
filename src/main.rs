use zero2prod::run_actix_backend;
use std::net::TcpListener;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to rand port");
    
        let port = listener.local_addr().unwrap().port();
        println!("<LocalHost>:<Port>: 127.0.0.1:{}", port);
    run_actix_backend(listener)?.await
}

