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

use super::release::Release;

pub trait Field {
    fn findByReleaseId<T>(i: i32, conn: &mut Connection) -> QueryResult<Vec<T>> {
        let rel = releases.filter(releases::id.eq(i))
            .select(Release::as_select())
            .get_result::<Release>(conn)?;

        Changelog::belonging_to(&rel)
            .select(T::as_select())
            .load::<T>(conn)
    }
}
