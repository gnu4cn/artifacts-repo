use actix_web::{web, HttpResponse};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::{
        response::ResponseBody,
        artifact::RepoDateDefconfig,
    },
    services::artifact_service,
};

// GET api/artiface/{a_id}
pub async fn find_by_id(
    a_id: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match artifact_service::find_by_id(a_id.into_inner(), &pool) {
        Ok(artifact) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, artifact))),
        Err(err) => Err(err),
    }
}

// POST api/artifact
pub async fn find_artifact_by_repo_date_defconfig(
    repo_date_defconfig: web::Json<RepoDateDefconfig>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match artifact_service::find_by_repo_date_defconfig(repo_date_defconfig.0, &pool) {
        Ok(release) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, release))),
        Err(err) => Err(err),
    }
}

