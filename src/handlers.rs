use actix_web::{get, HttpResponse, Responder};
use crate::models::GenericResponse;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "使用 Rust 与 Actix Web 构建简单的 CRUD。";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

