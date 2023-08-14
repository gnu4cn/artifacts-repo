use std::{env, io, default::Default};

use actix_web::{
    App,
    HttpServer,
    http,
    web,
    dev::Service,
    middleware::Logger,
};

use actix_cors::Cors;
use futures::FutureExt;

mod lib;
mod api;
mod models;
mod schema;
mod config;
mod error;
mod constants;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use config::db;

    dotenv::dotenv().expect("Failed to read .env file");

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = db::init_db_pool(&db_url);
    db::run_migration(&mut pool.get().unwrap());

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default() // allowed_origin return access-control-allow-origin: * by default
                .allowed_origin("http://127.0.0.1:20080")
                .allowed_origin("http://localhost:20080")
                .allowed_origin("https://download.senscomm.com")
                .send_wildcard()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap_fn(|req, srv| srv.call(req).map(|res| res))
            .configure(config::app::config_services)
    })
    .bind("0.0.0.0:20080")?
        .run()
        .await
}
