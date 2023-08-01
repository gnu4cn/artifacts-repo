use actix_web::{get, web, HttpResponse, Responder};

use crate::models::response::GenericResponse;

#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "使用 Rust 与 Actix Web 构建简单的 CRUD。";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    let resp_json = &GenericResponse {
        status: "success".to_string(),
        message: format!("你好，{}！", name),
    };
    HttpResponse::Ok().json(resp_json)
}

#[get("/hello")]
async fn greet_default() -> impl Responder {
    let resp_json = &GenericResponse {
        status: "success".to_string(),
        message: "Hello World!".to_string(),
    };
    HttpResponse::Ok().json(resp_json)
}
