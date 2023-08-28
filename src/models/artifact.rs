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
    schema::artifacts::{self, dsl::*},
};

use super::{
    release::Release,
    repository::{Repository, RepositoryDTO},
    changelog::Changelog,
    affected_file::AffectedFile,
    defconfig::{Defconfig, DefconfigDTO},
};

#[derive(Identifiable, Associations, PartialEq, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(belongs_to(Repository))]
#[diesel(belongs_to(Release))]
#[diesel(belongs_to(Defconfig))]
#[diesel(check_for_backend(pg::Pg))]
pub struct Artifact {
    pub id: i32,
    pub url: String,
    pub filesize: i64,
    pub build_log_url: Option<String>,
    pub repository_id: i32,
    pub release_id: i32,
    pub defconfig_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = artifacts)]
pub struct NewArtifact {
    pub url: String,
    pub filesize: i64,
    pub build_log_url: Option<String>,
    pub repository_id: i32,
    pub release_id: i32,
    pub defconfig_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewArtifactDTO {
    pub defconfig: String,
    pub url: Option<String>,
    pub filesize: Option<i64>,
    pub build_log_url: Option<String>,
}


impl Artifact {
    pub fn insert(
        rel_id: i32,
        def_id: i32,
        a: NewArtifact,
        conn: &mut Connection
    ) -> QueryResult<Artifact> {
        let new_artifact = NewArtifact {
            release_id: rel_id,
            defconfig_id: def_id,
            ..a
        };

        diesel::insert_into(artifacts)
            .values(&new_artifact)
            .returning(Artifact::as_returning())
            .get_result(conn)

    }

    pub fn find_artifacts_by_release_id(
        i: i32,
        conn: &mut Connection
    ) -> QueryResult<Vec<Artifact>> {
        let rel = Release::find_release_by_id(i, conn).unwrap();

        Artifact::belonging_to(&rel)
            .select(Artifact::as_select())
            .load::<Artifact>(conn)
    }
}


#[derive(Serialize, Deserialize)]
pub struct ArtifactDAO {
    pub artifact: Artifact,
    pub repository: Repository,
    pub defconfig: Defconfig,
    pub release: Release,
    pub changelogs: Vec<Changelog>,
    pub affected_files: Vec<AffectedFile>,
}

#[derive(Serialize, Deserialize)]
pub struct RepoDateDefconfig {
    pub repo: RepositoryDTO,
    pub date: NaiveDate,
    pub defconfig: DefconfigDTO,
}

impl ArtifactDAO {
    pub fn find_artifact_by_id(
        a_id: i32,
        conn: &mut Connection
    ) -> QueryResult<ArtifactDAO> {
        match artifacts.filter(id.eq(a_id)).get_result::<Artifact>(conn) {
            Ok(a) => {
                let r = Release::find_release_by_id(a.release_id, conn).unwrap();
                let r_id = r.id;

                Ok(ArtifactDAO {
                    artifact: a,
                    repository: Repository::find_by_id(r_id, conn).unwrap(),
                    defconfig: Defconfig::find_by_id(a.defconfig_id, conn).unwrap(),
                    release: r,
                    changelogs: Changelog::find_changlogs_by_release_id(r_id, conn).unwrap(),
                    affected_files: AffectedFile::find_affected_files_by_release_id(r_id, conn).unwrap(),
                })
            },
            Err(err) => Err(err),
        }
    }

    pub fn find_by_repo_date_defconfig(
        repo_date_defconfig: &RepoDateDefconfig,
        conn: &mut Connection
    ) -> QueryResult<ArtifactDAO> {
        let r = &repo_date_defconfig.repo;
        let d = repo_date_defconfig.date;
        let def = &repo_date_defconfig.defconfig;

        match Release::find_by_repo_date(r, d, conn) {
            Ok(rel) => {
                match Defconfig::find_by_dto(def, conn) {
                    Ok(def) => {
                        match artifacts.filter(release_id.eq(rel.id))
                            .filter(defconfig_id.eq(def.id))
                            .get_result::<Artifact>(conn) {
                                Ok(a) => {
                                    Ok(Self::find_artifact_by_id(a.id, conn).unwrap())
                                },
                                Err(err) => Err(err),
                            }
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => Err(err),
        }
    }
}
