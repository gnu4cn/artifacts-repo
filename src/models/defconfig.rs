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
    schema::defconfigs::{self, dsl::*},
};

#[derive(Identifiable, Queryable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = defconfigs)]
#[diesel(belongs_to(Repository))]
#[diesel(check_for_backend(pg::Pg))]
pub struct Defconfig {
    pub id: i32,
    pub config: String,
    pub repository_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[diesel(table_name = defconfigs)]
pub struct NewDefconfig {
    pub config: String,
    pub repository_id: i32,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DefconfigDTO {
    pub config: String,
}

impl Defconfig {
    pub fn insert(
        repo_id: i32,
        def: NewDefconfig,
        conn: &mut Connection
    ) -> QueryResult<Defconfig> {
        let new_defconfig = NewDefconfig {
            repository_id: repo_id,
            ..def
        };

        diesel::insert_into(defconfigs)
            .values(&new_defconfig)
            .returning(Defconfig::as_returning())
            .get_result(conn)
    }

    pub fn find_by_id(
        def_id: i32,
        conn: &mut Connection
    ) -> QueryResult<Defconfig> {
        defconfigs.filter(id.eq(def_id))
            .get_result::<Defconfig>(conn)
    }

    pub fn find_by_dto(
        dto: &DefconfigDTO,
        conn: &mut Connection
    ) -> QueryResult<Defconfig>{
        defconfigs.filter(config.eq(dto.config.to_string()))
            .get_result::<Defconfig>(conn)
    }
}
