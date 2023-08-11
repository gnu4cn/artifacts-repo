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

use super::{
    changelog::{Changelog, NewChangelog},
    artifact::{Artifact, NewArtifact},
    affected_file::{AffectedFile, NewAffectedFile},

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


#[derive(Serialize, Deserialize)]
pub struct ReleaseDTO {
    pub release: Release,
    pub changelogs: Vec<Changelog>,
    pub artifacts: Vec<Artifact>,
    pub affected_files: Vec<AffectedFile>,
}

impl ReleaseDTO {
    pub fn find_release_by_id(r_id: i32, conn: &mut Connection) -> QueryResult<ReleaseDTO> {

    }
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseForm {
    pub release: NewRelease,
    pub changelogs: Vec<NewChangelog>,
    pub artifacts: Vec<NewArtifact>,
    pub affected_files: Vec<NewAffectedFile>,
}
