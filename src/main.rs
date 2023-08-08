use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

mod services;
mod models;
mod schema;
mod config;

use crate::services::{
    hello::{
        greet,
        greet_default,
    },
    health_checker::health_checker_handler,
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
