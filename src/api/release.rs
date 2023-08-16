use actix_web::{web, HttpRequest, HttpResponse};

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
pub async fn save_rel(
    release_dto: web::Json<ReleaseDTO>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match release_service::save(release_dto.0, &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}

// GET api/release/{r_id}
pub async fn find_release_by_id(
    id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match release_service::find_by_id(id.into_inner(), &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}