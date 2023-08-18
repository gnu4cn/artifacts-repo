use actix_web::{http::header::HeaderValue, web};
use serde::{Deserialize, Serialize};
use serde_json::json;

use chrono::NaiveDate;

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::artifact::{Artifact, ArtifactDTO},
};

pub fn find_by_id(a_id: i32, pool: &web::Data<Pool>) -> Result<ArtifactDTO, ServiceError> {
    match Artifact::find_artifact_by_id(a_id, &mut pool.get().unwrap()) {
        Ok(artifact) => Ok(artifact),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("Artifact with id {} not found", a_id),
        }),
    }
}
