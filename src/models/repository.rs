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

use super::{
    release::{Release, ChannelDTO},
    artifact::{Artifact, DefconfigDTO},
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
    pub release_channels: Vec<ChannelDTO>,
    pub defconfigs: Vec<DefconfigDTO>,
    pub days: Vec<NaiveDate>,
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

    pub fn find_channel_dtos(
        repo_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Vec<ChannelDTO>> {
        let mut res: Vec<ChannelDTO> = Vec::new();

        match Release::find_distinct_release_channels(repo_id, conn) {
            Ok(channels) => {
                for c in channels {
                    match Release::count_release_by_channel(repo_id, &c, conn) {
                        Ok(n) => {
                            res.push(ChannelDTO {
                                channel: c,
                                count: n,
                            });
                        },
                        Err(err) => {},
                    }
                }
            },
            Err(err) => {},
        }

        Ok(res)
    }

    pub fn find_defconfig_dtos(
        repo_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Vec<DefconfigDTO>> {
        let mut res: Vec<DefconfigDTO> = Vec::new();

        match Artifact::find_distinct_defconfigs(repo_id, conn) {
            Ok(defconfigs) => {
                for conf in defconfigs {
                    match Artifact::count_artifact_by_defconfig(repo_id, &conf, conn) {
                        Ok(n) => {
                            res.push(DefconfigDTO{
                                defconfig: conf,
                                count: n,
                            });
                        },
                        Err(err) => {},
                    }
                }
            },
            Err(err) => {},
        }

        Ok(res)
    }

    pub fn find_brief_by_id (
        repo_id: i32,
        conn: &mut Connection
    ) -> QueryResult<RepositoryBriefDTO> {
        match Self::find_by_id(repo_id, conn) {
            Ok(r) => {
                Ok(RepositoryBriefDTO {
                    repo: r,
                    release_channels: Self::find_channel_dtos(repo_id, conn).unwrap(),
                    defconfigs: Self::find_defconfig_dtos(repo_id, conn).unwrap(),
                    days: Release::find_distinct_dates(repo_id, conn).unwrap(),
                })
            },
            Err(err) => Err(err),
        }
    }

    pub fn find_all_repository_brief(
        conn: &mut Connection
    ) -> QueryResult<Vec<RepositoryBriefDTO>> {
        let mut result: Vec<RepositoryBriefDTO> = Vec::new();

        for r in Self::find_all(conn).unwrap() {
            let repo_id = r.id;

            result.push(RepositoryBriefDTO {
                repo: r,
                release_channels: Self::find_channel_dtos(repo_id, conn).unwrap(),
                defconfigs: Self::find_defconfig_dtos(repo_id, conn).unwrap(),
                days: Release::find_distinct_dates(repo_id, conn).unwrap(),
            });
        }

        Ok(result)
    }
}
