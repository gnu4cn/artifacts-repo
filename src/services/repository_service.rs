use actix_web::web;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{
        repository::{Repository,RepositoryDTO, RepositoryBriefDTO, RepoTagDTO},
        artifact::Artifact,
        release::ReleaseDAO,
    },
};


pub fn find_repositories(
    pool: &web::Data<Pool>
) -> Result<Vec<Repository>, ServiceError> {
    match Repository::find_all(&mut pool.get().unwrap()) {
        Ok(res) => Ok(res),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No repository found. Err: {}", err),
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
                    error_message: format!("No defconfig found. Error: {}", err),
                }),
            }
        },
        Err(err) => Err(ServiceError::NotFound{
            error_message: format!("No repository found. Error: {}", err),
        }),
    }
}

pub fn find_repo_brief_by_id(
    repo_id: i32,
    pool: &web::Data<Pool>
) -> Result<RepositoryBriefDTO, ServiceError> {
    match RepositoryBriefDTO::find_by_repo_id(repo_id, &mut pool.get().unwrap()) {
        Ok(b) => Ok(b),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No repository brief found. Error: {}", err),
        }),
    }
}

pub fn find_all_repo_briefs(
    pool: &web::Data<Pool>
) -> Result<Vec<RepositoryBriefDTO>, ServiceError> {
    match RepositoryBriefDTO::find_all(&mut pool.get().unwrap()) {
        Ok(briefs) => Ok(briefs),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No repository brief found. Error: {}", err),
        }),
    }
}

pub fn find_tagged_releases_by_dto(
    repo_dto: RepositoryDTO,
    pool: &web::Data<Pool>
) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match Repository::find_tagged_releases_by_dto(&repo_dto, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound{
            error_message: format! ("No tagged release under repo: {}/{}. Error: {}", repo_dto.org, repo_dto.repo, err),
        }),
    }
}

pub fn find_release_by_repo_tag_dto(
    repo_tag: &RepoTagDTO,
    pool: &web::Data<Pool>
) -> Result<ReleaseDAO, ServiceError> {
    match Repository::find_release_by_repo_tag_dto(repo_tag, &mut pool.get().unwrap()) {
        Ok(r) => Ok(r),
        Err(err) => Err(ServiceError::NotFound{
            error_message: format! ("No release with tag {} under repository {:?} found. Error: {}",
                               repo_tag.tag, repo_tag.repo, err),
                }),
    }
}
