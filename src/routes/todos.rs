use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json},
};

use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, MySqlPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    status: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateNewTodo {
    title: String,
    description: Option<String>,
}

#[derive(Serialize)]
pub struct TypeDbError {
    error: String,
}

#[post("/todo/create")]
pub async fn create_todo(db: Data<MySqlPool>, body: Json<CreateNewTodo>) -> impl Responder {
    let response = sqlx::query("INSERT INTO todos(title,description) values(?,?)")
        .bind(&body.title)
        .bind(&body.description)
        .execute(&**db)
        .await;

    match response {
        Ok(id) => HttpResponse::Created().json(Todo {
            id: id.last_insert_id() as u32,
            title: body.title.clone(),
            description: body.description.clone(),
            status: "New".to_string(),
        }),
        Err(err) => HttpResponse::InternalServerError().json(TypeDbError {
            error: err.to_string(),
        }),
    }
}

#[get("/todos/all")]
pub async fn get_all_todos(db: Data<MySqlPool>) -> impl Responder {
    let response: Result<Vec<Todo>, Error> =
        sqlx::query_as("SELECT id, title, description, status from todos")
            .fetch_all(&**db)
            .await;

    match response {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(err) => HttpResponse::InternalServerError().json(TypeDbError {
            error: err.to_string(),
        }),
    }
}
