use actix_web::web;
use chrono::NaiveDate;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::release::{Release, ReleaseDAO, ReleaseDTO},
    models::repository::{RepositoryDTO, RepoDate},
};

pub fn save(
    rel: ReleaseDTO,
    pool: &web::Data<Pool>
) -> Result<ReleaseDAO, ServiceError> {
    match ReleaseDTO::save_release(rel, &mut pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::BadRequest {
            error_message: format! ("Error happened saving the release. Err: {}", message)
        }),
    }
}

pub fn find_by_id(
    id: i32,
    pool: &web::Data<Pool>
) -> Result<ReleaseDAO, ServiceError> {
    match ReleaseDAO::find_release_by_id(id, &mut pool.get().unwrap()) {
        Ok(release) => Ok(release),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("Release with id {} not found. Err: {}", id, err),
        }),
    }
}

pub fn find_all(
    pool: &web::Data<Pool>
) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match ReleaseDAO::find_all(&mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No release found. Err: {}", err),
        }),
    }
}

pub fn find_by_date(
    date: NaiveDate,
    pool: &web::Data<Pool>
) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match ReleaseDAO::find_releases_by_date(date, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No release found. Err: {}", err),
        }),
    }
}


pub fn find_releases_by_repository(
    r: &RepositoryDTO,
    pool: &web::Data<Pool>
) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match ReleaseDAO::find_by_repository(r, &mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("no release under repository {:?} found. Err: {}", r, err),
        }),
    }
}

pub fn find_by_repo_date(
    repo_date: RepoDate,
    pool: &web::Data<Pool>
) -> Result<ReleaseDAO, ServiceError> {
    let d = repo_date.date;
    let r = &repo_date.repo;

    match ReleaseDAO::find_by_repo_date(&repo_date, &mut pool.get().unwrap()) {
        Ok(rel) => Ok(rel),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("No release with repo {:?} and date {:?} found. Err: {}", r, d, err),
        }),
    }
}

pub fn find_days_released(
    pool: &web::Data<Pool>
) -> Result<Vec<NaiveDate>, ServiceError> {
    match Release::find_days_released(&mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("There is no release available. Err: {}", err),
        }),
    }
}

pub fn find_releases_of_today(
    pool: &web::Data<Pool>
) -> Result<Vec<ReleaseDAO>, ServiceError> {
    match ReleaseDAO::find_releases_of_today(&mut pool.get().unwrap()) {
        Ok(result) => Ok(result),
        Err(err) => Err(ServiceError::NotFound {
            error_message: format! ("There is no release today. Err: {}", err),
        }),
    }
}
