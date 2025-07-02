use actix_web::{App, HttpServer};

mod routes;
use routes::*;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 2099;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let server: actix_web::dev::Server = HttpServer::new(|| {
        App::new()
            .service(get_home)
            .service(hello_user)
            .service(create_new_user)
    })
    .bind((ADDR, PORT))?
    .run();

    println!("Server is running on {}:{}", ADDR, PORT);

    server.await
}
