use actix_web::web;
use log::info;

use crate::api::{
    hello::{
        greet,
        greet_default,
    },
    health_checker::health_checker_handler,
};

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("配置路由......");
    cfg.service();
}
