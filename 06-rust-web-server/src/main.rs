use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Rust Web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 서버를 시작합니다: http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello)) // "/"로 요청 오면 hello 함수 호출
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
