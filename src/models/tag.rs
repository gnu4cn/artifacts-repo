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
    schema::tags::{self, dsl::*},
    lib::logger::{self, Header},
};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Selectable, Debug)]
#[diesel(table_name = tags)]
#[diesel(check_for_backend(pg::Pg))]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub release_id: i32,
    pub repository_id: i32,
}

impl Tag {
    pub fn find_by_repository_id(
        repo_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Vec<Tag>> {
        tags.filter(repository_id.eq(repo_id))
            .order(id.desc())
            .load::<Tag>(conn)
    }

    pub fn find_by_release_id(
        rel_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Option<Tag>> {
        match tags.filter(release_id.eq(rel_id))
            .get_result::<Tag>(conn) {
                Ok(t) => Ok(Some(t)),
                Err(err) => Ok(None),
            }
    }

    pub fn find_by_dto(
        new_tag: &NewTag,
        conn: &mut Connection
    ) -> QueryResult<Tag> {
        tags.filter(name.eq(new_tag.name.to_string()))
            .filter(repository_id.eq(new_tag.repository_id))
            .get_result::<Tag>(conn)
    }

    pub fn find_by_repo_id_and_name(
        repo_id: i32,
        tag_name: &String,
        conn: &mut Connection
    ) -> QueryResult<Tag> {
        tags.filter(repository_id.eq(repo_id))
            .filter(name.eq(tag_name.to_string()))
            .get_result::<Tag>(conn)
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
    pub repository_id: i32,
    pub release_id: i32,
}

impl NewTag {
    pub fn save(
        repo_id: i32,
        rel_id: i32,
        tag: NewTag,
        conn: &mut Connection
    ) -> Result<Tag, String> {
        let tag_to_save = NewTag {
            repository_id: repo_id,
            release_id: rel_id,
            ..tag
        };


        match Tag::find_by_dto(&tag_to_save, conn) {
            Ok(t) => {
                logger::log(Header::WARNING, &format! ("{:?} existed already.", &tag_to_save.name));
                Err(format! ("Tag '{}' is already existed.", &tag_to_save.name))
            },
            Err(err) => {
                Ok(diesel::insert_into(tags)
                    .values(&tag_to_save)
                    .returning(Tag::as_returning())
                    .get_result::<Tag>(conn).unwrap())
            },
        }
    }
}
