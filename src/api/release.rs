use actix_web::{web, HttpResponse};

use chrono::NaiveDate;

use crate::{
    config::db::Pool,
    error::ServiceError,
    constants,
    models::{
        response::ResponseBody,
        release::ReleaseDTO,
        repository::{RepoDate, RepositoryDTO},
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

// GET api/release
pub async fn find_all(
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_all(&pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// POST api/release
pub async fn find_by_repo_date(
    repo_date: web::Json<RepoDate>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_by_repo_date(repo_date.0, &pool) {
        Ok(rel) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, rel))),
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

// POST api/release/repository
pub async fn find_releases_by_repository (
    repo: web::Json<RepositoryDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_releases_by_repository(&repo.0, &pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// GET api/release/days
pub async fn find_days_released (
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_days_released(&pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}
