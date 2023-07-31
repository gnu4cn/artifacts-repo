use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use crate::handlers::health_checker_handler;

mod handlers;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("\nServer started successfully...\n");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 20080))?
        .run()
        .await
}
