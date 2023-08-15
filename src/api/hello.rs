use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    error::ServiceError,
    models::response::GenericResponse,
};

// GET api/hello/{name}
pub async fn greet(name: web::Path<String>) -> Result<HttpResponse, ServiceError> {
    let resp_json = &GenericResponse {
        status: "success".to_string(),
        message: format!("你好，{}！", name),
    };
    Ok(HttpResponse::Ok().json(resp_json))
}

// GET api/hello
pub async fn greet_default() -> Result<HttpResponse, ServiceError> {
    let resp_json = &GenericResponse {
        status: "success".to_string(),
        message: "Hello World!".to_string(),
    };
    Ok(HttpResponse::Ok().json(resp_json))
}
