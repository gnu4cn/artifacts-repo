use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    models::response::GenericResponse,
    error::ServiceError,
};

pub async fn health_checker_handler() -> Result<HttpResponse, ServiceError> {
    const MESSAGE: &str = "使用 Rust 与 Actix Web 构建简单的 CRUD。";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(HttpResponse::Ok().json(response_json))
}
