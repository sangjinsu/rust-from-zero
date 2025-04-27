use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use serde::{Deserialize, Serialize};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust Web!")
}

async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[derive(Deserialize)]
struct Info {
    username: String,
}

async fn query_info(query: web::Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Query username: {}", query.username))
}

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[derive(Serialize)]
struct UserResponse {
    message: String,
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    let response = UserResponse {
        message: format!(
            "{}님, 나이 {}살 유저가 생성되었습니다.",
            user.name, user.age
        ),
    };
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 서버 시작: http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/greet/{name}", web::get().to(greet))
            .route("/query", web::get().to(query_info))
            .route("/users", web::post().to(create_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
