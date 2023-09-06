use diesel::{
    prelude::*,
    Identifiable,
    Insertable,
    Queryable,
    pg,
};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::{
    config::db::Connection,
    schema::repositories::{self, dsl::*},
};


#[derive(Identifiable, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = repositories)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Repository {
    pub id: i32,
    pub org: String,
    pub repo: String,
}


#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[diesel(table_name = repositories)]
pub struct RepositoryDTO {
    pub org: String,
    pub repo: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoDate {
    pub repo: RepositoryDTO,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct RepositoryBriefDTO {
    pub repo: Repository,
    pub release_channels: Vec<String>,
    pub defconfigs: Vec<String>,
}

impl Repository {
    pub fn insert(
        r: RepositoryDTO,
        conn: &mut Connection
    ) -> QueryResult<Repository> {
        match Self::find_by_dto(&r, conn) {
            Ok(r) => Ok(r),
            Err(err) => {
                diesel::insert_into(repositories)
                    .values(&r)
                    .returning(Repository::as_returning())
                    .get_result(conn)
            }
        }
    }

    pub fn find_by_dto(
        dto: &RepositoryDTO,
        conn: &mut Connection
    ) -> QueryResult<Repository> {
        repositories.filter(org.eq(dto.org.to_string()))
            .filter(repo.eq(dto.repo.to_string()))
            .get_result::<Repository>(conn)
    }

    pub fn find_by_id(
        r_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Repository> {
        repositories.filter(id.eq(r_id))
            .get_result::<Repository>(conn)
    }

    pub fn find_all(
        conn: &mut Connection
    ) -> QueryResult<Vec<Repository>> {
        repositories.order(org.asc())
            .order(repo.asc())
            .load::<Repository>(conn)
    }
}
