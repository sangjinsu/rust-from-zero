mod handlers;

use actix_web::{App, HttpServer, web};

use handlers::{
    greet_handler::greet, hello_handler::hello, query_handler::query_info,
    user_handler::create_user,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 서버 시작: http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .service(query_info)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
