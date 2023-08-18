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
    schema::artifacts::{self, dsl::*},
    error::ServiceError,
};

use super::{
    release::Release,
    changelog::Changelog,
    affected_file::AffectedFile,
};

#[derive(Identifiable, Associations, PartialEq, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(belongs_to(Release))]
#[diesel(check_for_backend(pg::Pg))]
pub struct Artifact {
    pub id: i32,
    pub defconfig: String,
    pub filename: String,
    pub filesize: i64,
    pub build_log_path: Option<String>,
    pub release_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = artifacts)]
pub struct NewArtifact {
    pub defconfig: String,
    pub filename: String,
    pub filesize: i64,
    pub build_log_path: Option<String>,
    pub release_id: i32,
}


#[derive(Serialize, Deserialize)]
pub struct ArtifactDTO {
    pub artifact: Artifact,
    pub release: Release,
    pub changelogs: Vec<Changelog>,
    pub affected_files: Vec<AffectedFile>,
}

impl Artifact {
    pub fn insert(rel_id: i32, a: NewArtifact, conn: &mut Connection) -> QueryResult<Artifact> {
        let new_artifact = NewArtifact {
            release_id: rel_id,
            ..a
        };

        diesel::insert_into(artifacts)
            .values(&new_artifact)
            .returning(Artifact::as_returning())
            .get_result(conn)

    }

    pub fn find_artifacts_by_release_id(i: i32, conn: &mut Connection) -> QueryResult<Vec<Artifact>> {
        let rel = Release::find_release_by_id(i, conn).unwrap();

        Artifact::belonging_to(&rel)
            .select(Artifact::as_select())
            .load::<Artifact>(conn)
    }

    pub fn find_artifact_by_id(a_id: i32, conn: &mut Connection) -> QueryResult<ArtifactDTO> {
        match artifacts.filter(id.eq(a_id)).get_result::<Artifact>(conn) {
            Ok(a) => {
                let r_id = a.release_id;
                let r = Release::find_release_by_id(r_id, conn).unwrap();

                Ok(ArtifactDTO {
                    artifact: a,
                    release: r,
                    changelogs: Changelog::find_changlogs_by_release_id(r_id, conn).unwrap(),
                    affected_files: AffectedFile::find_affected_files_by_release_id(r_id, conn).unwrap(),
                })
            },
            Err(err) => Err(err),
        }
    }
}
