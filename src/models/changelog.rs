use diesel::{
    prelude::*,
    Identifiable,
    Insertable,
    Queryable,
    pg,
};
use serde::{Deserialize, Serialize};

use crate::{
    config::db::Connection,
    schema::changelogs::{self, dsl::*},
    schema::releases::{self, dsl::*},
    error::ServiceError,
};

use super::{
    release::Release,
    field_trait::Field,
};

#[derive(Identifiable, Associations, PartialEq, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(belongs_to(Release))]
#[diesel(check_for_backend(pg::Pg))]
pub struct Changelog {
    pub id: i32,
    pub commit_id: String,
    pub commit_comment: String,
    pub commited_by: String,
    pub release_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = changelogs)]
pub struct NewChangelog {
    pub commit_id: String,
    pub commit_comment: String,
    pub commited_by: String,
    pub release_id: i32,
}

impl Field for Changelog {}

impl Changelog {
    pub fn insert(rel: Release, changelog: NewChangelog, conn: &mut Connection) -> QueryResult<Changelog> {
        let new_changelog = NewChangelog {
            release_id: rel.id,
            ..changelog
        };

        diesel::insert_into(changelogs)
            .values(&new_changelog)
            .returning(Changelog::as_returning())
            .get_result(conn)

    }
}
