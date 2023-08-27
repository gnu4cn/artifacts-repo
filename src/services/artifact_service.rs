use actix_web::web;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::artifact::{ArtifactDTO, RepoDateDefconfig},
};

pub fn find_by_id(a_id: i32, pool: &web::Data<Pool>) -> Result<ArtifactDTO, ServiceError> {
    match ArtifactDTO::find_artifact_by_id(a_id, &mut pool.get().unwrap()) {
        Ok(artifact) => Ok(artifact),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("Artifact with id {} not found", a_id),
        }),
    }
}

pub fn find_by_repo_date_defconfig(
    repo_date_defconfig: RepoDateDefconfig,
    pool: &web::Data<Pool>
) -> Result<ArtifactDTO, ServiceError> {
    match ArtifactDTO::find_by_repo_date_defconfig(
        &repo_date_defconfig,
        &mut pool.get().unwrap()
    ) {
        Ok(a) => Ok(a),
        Err(err) => Err(
            ServiceError::NotFound {
                error_message: format! ("Artifact with repo name {:?}, date {}, and defconfig {} not found", repo_date_defconfig.repo, repo_date_defconfig.date, repo_date_defconfig.defconfig),
            }),
    }
}
