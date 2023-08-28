use actix_web::{web, HttpResponse};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::ResponseBody,
        repository::{RepoDate, Repository},
    },
    services::repository_service,
};

// GET api/repository
pub async fn find_repositories (
    pool: web::Data<Pool>
)-> Result<HttpResponse, ServiceError> {
    match repository_service::find_repositories(&pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}
