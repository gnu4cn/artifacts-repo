use actix_web::web;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{
        repository::{Repository,RepositoryDTO},
        artifact::Artifact,
    },
};

pub fn find_repositories(
    pool: &web::Data<Pool>
) -> Result<Vec<Repository>, ServiceError> {
    match Repository::find_all(&mut pool.get().unwrap()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No repository found"),
        }),
    }
}

pub fn find_repository_defconfigs(
    repo_dto: RepositoryDTO,
    pool: &web::Data<Pool>
) -> Result<Vec<String>, ServiceError> {
    match Repository::find_by_dto(&repo_dto, &mut pool.get().unwrap()) {
        Ok(r) => {
            match Artifact::find_distinct_defconfigs(r.id, &mut pool.get().unwrap()) {
                Ok(defconfigs) => Ok(defconfigs),
                Err(err) => Err(ServiceError::NotFound{
                    error_message: format!("No defconfig found, error: {}", err),
                }),
            }
        },
        Err(err) => Err(ServiceError::NotFound{
            error_message: format!("No repository found, error: {}", err),
        }),
    }
}
