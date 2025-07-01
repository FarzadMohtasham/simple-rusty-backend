use actix_web::{
    App, HttpServer, Responder, get,
    http::StatusCode,
    web::{Json, Path},
};
use serde::Serialize;

const ADDR: &str = "127.0.0.1";
const PORT: u16 = 2099;

#[get("/home")]
async fn get_home() -> impl Responder {
    let response = "Hello Home!";
    response
}

#[get("/home/{first_name}/{last_name}")]
async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let response = User::new(params.0.clone(), params.1.clone());
    (Json(response), StatusCode::OK)
}

#[derive(Serialize)]
struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new().service(get_home).service(hello_user))
        .bind((ADDR, PORT))?
        .run();

    println!("Server is running on {}:{}", ADDR, PORT);

    server.await
}
