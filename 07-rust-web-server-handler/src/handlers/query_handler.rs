use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub username: String,
}

#[actix_web::get("/query")]
pub async fn query_info(query: web::Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Query username: {}", query.username))
}
