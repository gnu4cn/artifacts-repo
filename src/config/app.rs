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
    artifact,
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
            )
            .service(
                web::resource("/release")
                .route(web::post().to(release::find_releases_by_repository)),
            ),
        )
        .service(
            web::scope("/release")
            .service(
                web::resource("")
                .route(web::get().to(release::find_all)),
            )
            .service(
                web::resource("/repo/date")
                .route(web::post().to(release::find_by_repo_date)),
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
        )
        .service(
            web::scope("/artifact")
            .service(
                web::resource("")
                .route(web::post().to(artifact::find_artifact_by_repo_date_defconfig)),
            )
            .service(
                web::resource("/{a_id}")
                .route(web::get().to(artifact::find_by_id)),
            ),
        ),
    );
}
