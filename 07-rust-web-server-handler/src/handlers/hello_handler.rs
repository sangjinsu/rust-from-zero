use actix_web::{HttpResponse, Responder};

#[actix_web::get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust Web!")
}
