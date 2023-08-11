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
    schema::releases::{self, dsl::*},
    error::ServiceError,
};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Release {
    pub id: i32,
    pub repo_fullname: String,
    pub diffs_url: Option<String>,
    pub released_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = releases)]
pub struct NewRelease {
    pub repo_fullname: String,
    pub diffs_url: String,
}

impl Release {
    pub fn insert(new_release: NewRelease, conn: &mut Connection) -> QueryResult<Release> {
        diesel::insert_into(releases)
            .values(&new_release)
            .returning(Release::as_returning())
            .get_result(conn)
    }

    pub fn find_release_by_date(date: NaiveDate, conn: &mut Connection) -> QueryResult<Release> {
        releases.filter(released_at.eq(&date)).get_result::<Release>(conn)
    }

    pub fn find_release_by_id(r_id: i32, conn: &mut Connection) -> QueryResult<Release> {
        releases.filter(id.eq(r_id)).get_result::<Release>(conn)
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Release>> {
        releases.order(id.asc()).load::<Release>(conn)
    }
}
