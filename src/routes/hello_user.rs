use actix_web::{
    Responder, get,
    http::StatusCode,
    web::{Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::routes::logging;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
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

#[get("/home/{first_name}/{last_name}")]
pub async fn hello_user(params: Path<(String, String)>) -> impl Responder {
    let firstname: &String = &params.0;
    let lastname: &String = &params.1;

    logging(format!("/home/first_name({})/last_name({})", firstname, lastname).as_str());

    let response: User = User::new(firstname.clone(), lastname.clone());
    (Json(response), StatusCode::OK)
}
