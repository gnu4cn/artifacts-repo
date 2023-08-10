use diesel::{prelude::*, Identifiable, Insertable, Queryable, dsl::{date, now}};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::releases::{self, dsl::*},
    models::db_enum::ChannelType,
    error::ServiceError,
};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Insertable)]
pub struct Release {
    pub id: String,
    pub channel: ChannelType,
    pub repo_fullname: String,
    pub diffs_url: String,
    pub released_at: NaiveDate,
}

impl Release {
    pub fn create(new_release: Release, conn: &mut Connection) -> Result<Release, String> {
        let release = Release {
            id: Uuid::new_v4().to_string(),
            released_at: diesel::select(date(now)).first(conn).unwrap(),
            ..new_release
        };
        diesel::insert_into(releases)
            .values(&release)
            .execute(conn)
            .expect("Failes to create new release");
        Ok(release)
    }

    pub fn find_release_by_date(date: NaiveDate, conn: &mut Connection) -> QueryResult<Release> {
        releases.filter(released_at.eq(date)).get_result::<Release>(conn)
    }
}
