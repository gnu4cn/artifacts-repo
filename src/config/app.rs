use actix_web::web;
use log::info;

use crate::api::{
    hello::{
        greet,
        greet_default,
    },
    health_checker::health_checker_handler,
    ping_controller::ping,
    release::{save_rel, find_release_by_id},
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("配置路由......");

    cfg.service(
        web::scope("/api")
        .service(ping)
        .service(
            web::resource("/healthchecker").route(web::get().to(health_checker_handler)),
        )
        .service(
            web::scope("/hello")
            .service(
                web::resource("")
                .route(web::get().to(greet_default)),
            )
            .service(
                web::resource("/{name}")
                .route(web::get().to(greet)),
            ),
        )
        .service(
            web::scope("/release")
            .service(
                web::resource("/new")
                .route(web::post().to(save_rel)),
            )
            .service(
                web::resource("/{r_id}")
                .route(web::get().to(find_release_by_id)),
            ),
        ),
    );
}
