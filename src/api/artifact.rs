use actix_web::{web, HttpRequest, HttpResponse};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::response::ResponseBody,
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

