use actix_web::web;

use crate::{
    config::db::Pool,
    error::ServiceError,
    models::repository::Repository,
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

