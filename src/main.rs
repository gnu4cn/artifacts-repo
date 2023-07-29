use actix_web::{web, App, HttpServer};

pub mod handlers;

use crate::handlers::*;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!\n" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 20080))?
        .run()
        .await
}
