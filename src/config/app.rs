use actix_web::web;
use log::info;

use crate::api::{
    hello::{
        greet,
        greet_default,
    },
    health_checker::health_checker_handler,
    ping_controller::ping,
    release,
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
            web::scope("repository")
            .service(
                web::resource("")
                .route(web::get().to(release::find_repositories)),
            ),
        )
        .service(
            web::scope("/release")
            .service(
                web::resource("")
                .route(web::get().to(release::find_all)),
            )
            .service(
                web::resource("/new")
                .route(web::post().to(release::save)),
            )
            .service(
                web::resource("/{r_id}")
                .route(web::get().to(release::find_by_id)),
            )
            .service(
                web::resource("/date/{date}")
                .route(web::get().to(release::find_by_date)),
            ),
        ),
    );
}
