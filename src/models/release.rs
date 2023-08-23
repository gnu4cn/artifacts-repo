use diesel::{
    result::Error,
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
    pub org: String,
    pub repo: String,
    pub diffs_url: Option<String>,
    pub released_at: NaiveDate,
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = releases)]
pub struct NewRelease {
    pub org: String,
    pub repo: String,
    pub diffs_url: Option<String>,
}

impl Release {
    pub fn insert(
        new_release: NewRelease,
        conn: &mut Connection
    ) -> QueryResult<Release> {
        diesel::insert_into(releases)
            .values(&new_release)
            .returning(Release::as_returning())
            .get_result(conn)
    }

    pub fn find_releases_by_date(
        date: NaiveDate,
        conn: &mut Connection
    ) -> QueryResult<Vec<Release>> {
        releases.filter(released_at.eq(&date))
            .order(repo.asc())
            .order(id.desc())
            .load::<Release>(conn)
    }

    pub fn find_release_by_id(
        r_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Release> {
        releases.filter(id.eq(r_id))
            .get_result::<Release>(conn)
    }

    pub fn find_by_repo_date(
        r: String,
        d: NaiveDate,
        conn: &mut Connection
    ) -> QueryResult<Release> {
        releases.filter(repo.eq(r).and(released_at.eq(d)))
            .get_result::<Release>(conn)
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<Release>> {
        releases.order(released_at.desc())
            .order(id.desc())
            .load::<Release>(conn)
    }

    pub fn find_repositories(conn: &mut Connection) -> QueryResult<Vec<String>> {
        releases.select(repo)
            .order(repo.asc())
            .distinct()
            .load::<String>(conn)
    }

    pub fn find_by_repository(
        r: String,
        conn: &mut Connection
    ) -> QueryResult<Vec<Release>> {
        releases.filter(repo.eq(r))
            .order(released_at.desc())
            .order(id.desc())
            .load::<Release>(conn)
    }
}


#[derive(Serialize, Deserialize)]
pub struct ReleaseDAO {
    pub release: Release,
    pub changelogs: Vec<Changelog>,
    pub artifacts: Vec<Artifact>,
    pub affected_files: Vec<AffectedFile>,
}

#[derive(Serialize, Deserialize)]
pub struct RepoDate {
    pub repo: String,
    pub date: NaiveDate,
}

impl ReleaseDAO {
    pub fn find_release_by_id(
        r_id: i32,
        conn: &mut Connection
    ) -> QueryResult<ReleaseDAO> {
        match Release::find_release_by_id(r_id, conn) {
            Ok(rel) => Ok(ReleaseDAO {
                release: rel,
                changelogs: Changelog::find_changlogs_by_release_id(r_id, conn).unwrap(),
                artifacts: Artifact::find_artifacts_by_release_id(r_id, conn).unwrap(),
                affected_files: AffectedFile::find_affected_files_by_release_id(r_id, conn).unwrap(),
            }),
            Err(err) => Err(err),
        }
    }

    fn get_dao_list_by_release_list(
        release_list: Vec<Release>,
        conn: &mut Connection
    ) -> Vec<ReleaseDAO> {
        let mut result: Vec<ReleaseDAO> = Vec::new();

        for r in release_list {
            let r_id = r.id;

            result.push(ReleaseDAO {
                release: r,
                changelogs: Changelog::find_changlogs_by_release_id(r_id, conn).unwrap(),
                artifacts: Artifact::find_artifacts_by_release_id(r_id, conn).unwrap(),
                affected_files: AffectedFile::find_affected_files_by_release_id(r_id, conn).unwrap(),
            });
        }

        result
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<ReleaseDAO>> {
        let release_list = Release::find_all(conn).unwrap();
        Ok(Self::get_dao_list_by_release_list(release_list, conn))
    }


    pub fn find_releases_by_date(
        date: NaiveDate,
        conn: &mut Connection
    ) -> QueryResult<Vec<ReleaseDAO>> {
        let release_list = Release::find_releases_by_date(date, conn).unwrap();
        Ok(Self::get_dao_list_by_release_list(release_list, conn))
    }

    pub fn find_by_repository(
        r: &String,
        conn: &mut Connection
    ) -> QueryResult<Vec<ReleaseDAO>> {
        let release_list = Release::find_by_repository(r.to_string(), conn).unwrap();
        Ok(Self::get_dao_list_by_release_list(release_list, conn))
    }

    pub fn find_by_repo_date (
        repo_date: &RepoDate,
        conn: &mut Connection
    ) -> QueryResult<ReleaseDAO> {
        let r = &repo_date.repo;
        let d = repo_date.date;

        match Release::find_by_repo_date(r.to_string(), d, conn) {
            Ok(r) => {
                Self::find_release_by_id(r.id, conn)
            },
            Err(err) => Err(err),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReleaseDTO {
    pub release: NewRelease,
    pub changelogs: Vec<NewChangelog>,
    pub artifacts: Vec<NewArtifact>,
    pub affected_files: Vec<NewAffectedFile>,
}

impl ReleaseDTO {
    pub fn save_release(
        rel: ReleaseDTO,
        conn: &mut Connection
    ) -> QueryResult<ReleaseDAO> {
        let rel_saved = Release::insert(rel.release, conn).unwrap();

        let mut saved_changelogs: Vec<Changelog> = Vec::new();
        for c in rel.changelogs {
            saved_changelogs.push(Changelog::insert(rel_saved.id, c, conn).unwrap());
        }

        let mut saved_artifacts: Vec<Artifact> = Vec::new();
        for a in rel.artifacts {
            saved_artifacts.push(Artifact::insert(rel_saved.id, a, conn).unwrap());
        }

        let mut saved_affected_files: Vec<AffectedFile> = Vec::new();
        for a in rel.affected_files {
            saved_affected_files.push(AffectedFile::insert(rel_saved.id, a, conn).unwrap());
        }

        Ok(ReleaseDAO{
            release: rel_saved,
            changelogs: saved_changelogs,
            artifacts: saved_artifacts,
            affected_files: saved_affected_files,
        })
    }
}
