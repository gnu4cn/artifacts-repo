use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

use crate::{
    config::db::Connection,
    schema::releases::{self, dsl::*},
    error::ServiceError,
};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Release {
    pub id: i64,
    pub repo_fullname: String,
    pub diffs_url: String,
    pub released_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = releases)]
pub struct ReleaseDTO {
    pub repo_fullname: String,
    pub diffs_url: String,
}

impl Release {
    pub fn create(new_release: ReleaseDTO, conn: &mut Connection) -> QueryResult<usize> {
        diesel::insert_into(releases)
            .values(&new_release)
            .execute(conn)
    }

    pub fn find_release_by_date(date: NaiveDate, conn: &mut Connection) -> QueryResult<Release> {
        releases.filter(released_at.eq(&date)).get_result::<Release>(conn)
    }
}
