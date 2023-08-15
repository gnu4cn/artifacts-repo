use actix_web::{http::header::HeaderValue, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

crate::models::{
    config::db::Pool,
    constants,
    error::ServiceError,
    release::{ReleaseDAO, ReleaseDTO},
};

pub fn save(rel:: ReleaseDTO, pool: &web::Data<Pool>) -> Result<ReleaseDAO, ServiceError> {
    match ReleaseDTO::save_release(rel, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest {
            error_message: message,
        }),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<Pool>) -> Result<ReleaseDAO, ServiceError> {
    match ReleaseDAO::find_release_by_id(id, &mut pool.get().unwrap()) {
        Ok(release) => Ok(release),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("Release with id {} not found", id),
        }),
    }
}
