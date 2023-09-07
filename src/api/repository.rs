use actix_web::{web, HttpResponse};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::ResponseBody,
        repository::{RepoDate, Repository, RepositoryDTO},
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

// POST api/repository/defconfig
pub async fn find_distinct_defconfigs (
    repo: web::Json<RepositoryDTO>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match repository_service::find_repository_defconfigs(repo.0, &pool) {
        Ok(result) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, result))),
        Err(err) => Err(err),
    }
}

// GET api/repository/brief/{repo_id}
pub async fn find_repo_brief_by_id(
    repo_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match repository_service::find_repo_brief_by_id(repo_id.into_inner(), &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}


// GET api/repository/brief
pub async fn find_all_repo_brief(
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match repository_service::find_all_repo_briefs(&pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}
