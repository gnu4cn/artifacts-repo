use actix_web::{get, web, HttpResponse, Responder};

use crate::models::response::GenericResponse;

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
