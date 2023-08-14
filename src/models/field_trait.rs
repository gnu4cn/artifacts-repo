use diesel::{
    prelude::*,
};

use super::release::Release;

use crate::{
    schema::releases::{self, dsl::*},
    config::db::Connection,
};

pub trait Field {
    fn find_by_release_id<T>(i: i32, conn: &mut dyn Connection) -> QueryResult<Vec<T>> {
        let rel = releases.filter(releases::id.eq(i))
            .select(Release::as_select())
            .get_result::<Release>(conn)?;

        T::belonging_to(&rel)
            .select(T::as_select())
            .load::<T>(conn)
    }
}
