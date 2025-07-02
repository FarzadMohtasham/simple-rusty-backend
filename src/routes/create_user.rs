use actix_web::{Responder, http::StatusCode, post, web::Json};
use serde::Serialize;

use crate::routes::{User, logging};

#[derive(Serialize)]
struct CreateUserResponse {
    id: u32,
    user: User,
}

#[post("/users/create")]
pub async fn create_new_user(user: Json<User>) -> impl Responder {
    logging("POST: /users/create");

    (
        Json(CreateUserResponse {
            id: 0,
            user: user.0,
        }),
        StatusCode::CREATED,
    )
}
