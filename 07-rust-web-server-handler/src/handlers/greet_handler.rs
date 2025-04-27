use actix_web::{web, HttpResponse, Responder};

#[actix_web::get("/greet/{name}")]
pub async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}
