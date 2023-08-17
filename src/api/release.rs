use actix_web::{web, HttpRequest, HttpResponse};

use chrono::NaiveDate;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::ResponseBody,
        release::{ReleaseDAO, ReleaseDTO},
    },
    services::release_service,
};


// POST api/release/new
pub async fn save(
    release_dto: web::Json<ReleaseDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match release_service::save(release_dto.0, &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}

// GET api/release/{r_id}
pub async fn find_by_id(
    id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_by_id(id.into_inner(), &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}

// GET api/release/all
pub async fn find_all(
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_all(&pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// GET api/release/date/{date}
pub async fn find_by_date(
    date: web::Path<NaiveDate>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_by_date(date.into_inner(), &pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// GET api/repository
pub async fn find_repositories (
    pool: web::Data<Pool>
)-> Result<HttpResponse, ServiceError> {
    match release_service::find_repositories(&pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// GET api/release/repository/{repo}
pub async fn find_releases_by_repository (
    repo: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_releases_by_repository(repo.into_inner(), &pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}
