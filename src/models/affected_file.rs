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
    schema::affected_files::{self, dsl::*},
    schema::releases::{self, dsl::*},
};

use super::release::Release;

#[derive(Identifiable, Associations, PartialEq, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(belongs_to(Release))]
#[diesel(check_for_backend(pg::Pg))]
pub struct AffectedFile {
    pub id: i32,
    pub file_edit_type: String,
    pub file_path: String,
    pub release_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = affected_files)]
pub struct NewAffectedFile {
    pub file_edit_type: String,
    pub file_path: String,
    pub release_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewAffectedFileDTO {
    pub file_edit_type: String,
    pub file_path: String,
}

impl AffectedFile {
    pub fn insert(rel_id: i32, f: NewAffectedFile, conn: &mut Connection) -> QueryResult<AffectedFile> {
        let new_affected_file = NewAffectedFile {
            release_id: rel_id,
            ..f
        };

        diesel::insert_into(affected_files)
            .values(&new_affected_file)
            .returning(AffectedFile::as_returning())
            .get_result(conn)

    }

    pub fn find_affected_files_by_release_id(i: i32, conn: &mut Connection) -> QueryResult<Vec<AffectedFile>> {
        let rel = Release::find_release_by_id(i, conn).unwrap();

        AffectedFile::belonging_to(&rel)
            .select(AffectedFile::as_select())
            .load::<AffectedFile>(conn)
    }
}
