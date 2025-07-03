use actix_web::{App, HttpServer};

mod routes;
use routes::*;

mod database;
use database::*;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 2099;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = database_connection()
        .await
        .expect("Failed to connect database");

    println!("Database connection established");

    let server: actix_web::dev::Server = HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(get_home)
            .service(hello_user)
            .service(create_new_user)
            .service(create_todo)
            .service(get_all_todos)
    })
    .bind((ADDR, PORT))?
    .run();

    println!("Server is running on {}:{}", ADDR, PORT);

    server.await
}
