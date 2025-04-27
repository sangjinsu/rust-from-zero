use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub message: String,
}

#[actix_web::post("/users")]
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    let response = UserResponse {
        message: format!("{}님, 나이 {}살 유저가 생성되었습니다.", user.name, user.age),
    };
    HttpResponse::Ok().json(response)
}
