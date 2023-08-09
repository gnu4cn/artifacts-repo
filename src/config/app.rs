use actix_web::web;
use log::info;

use crate::api::{
    hello::{
        greet,
        greet_default,
    },
    health_checker::health_checker_handler,
    ping_controller::ping,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("配置路由......");

    cfg.service(
        web::scope("/api")
        .service(ping)
        .service(
            web::resource("/healthchecker").route(web::get().to(health_checker_handler)),
        ),
        web::scope("/hello")
        .service(ping)
        .service(
            web::resource("")
            .route(web::get().to(greet_default)),
        )
        .service(
            web::resource("/{name}")
            .route(web::get().to(greet)),
        ),
    );
}
