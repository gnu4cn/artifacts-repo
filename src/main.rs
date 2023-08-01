use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod handlers;
mod models;
mod schema;
mod config;

use crate::handlers::{
    health_checker_handler,
    greet,
    greet_default,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    println!("Server started successfully...");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .service(greet)
            .service(greet_default)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 20080))?
        .run()
        .await
}
