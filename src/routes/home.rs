use actix_web::{Responder, get};

use crate::routes::logging;

#[get("/home")]
pub async fn get_home() -> impl Responder {
    logging("/home");
    let response: &'static str = "Hello Home!";
    response
}
