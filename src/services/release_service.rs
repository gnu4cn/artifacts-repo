use actix_web::{http::header::HeaderValue, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::release::{ReleaseDAO, ReleaseDTO},
};

pub fn save(rel: ReleaseDTO, pool: &web::Data<Pool>) -> Result<ReleaseDAO, ServiceError> {
    match ReleaseDTO::save_release(rel, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest {
            error_message: "Error happened saving the release".to_string(),
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

pub fn find_all(pool: &web::Data<Pool>) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match ReleaseDAO::find_all(&mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No release found"),
        }),
    }
}
