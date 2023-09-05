use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::Integer;
use diesel::QueryId;

use crate::constants::{
    EMPTY_STR,
    DEFAULT_PER_PAGE,
    MESSAGE_OK,
};

use super::response::Page;

pub trait SortingAndPaging: Sized {
    fn paginate(self, page: i32) -> SortedAndPaginated<Self>;
}

impl<T> SortingAndPaging for T {
    fn paginate(self, page: i32) -> SortedAndPaginated<Self> {
        SortedAndPaginated {
            query: self,
            sort_by: EMPTY_STR.to_string(),
            sort_direction: EMPTY_STR.to_string(),
            per_page: DEFAULT_PER_PAGE,
            page,
            offset: (page - 1) * DEFAULT_PER_PAGE,
        }
    }
}

#[derive(Debug, Clone, QueryId)]
pub struct SortedAndPaginated<T> {
    query: T,
    sort_by: String,
    sort_direction: String,
    page: i32,
    per_page: i32,
    offset: i32,
}

impl<T> SortedAndPaginated<T> {
    pub fn per_page(self, per_page: i32) -> Self {
        SortedAndPaginated { per_page, offset: (self.page - 1) * per_page, ..self }
    }

    pub fn sort(self, sort_by: String, sort_direction: String) -> Self {
        SortedAndPaginated {
            sort_by,
            sort_direction,
            ..self
        }
    }

    pub fn load_and_count_items<'a, U>(self, conn: &mut PgConnection) -> QueryResult<Page<U>>
    where
        Self: LoadQuery<'a, PgConnection, (U, i32)>,
    {
        let page = self.page;
        let per_page = self.per_page;
        let results = self.load::<(U, i32)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        Ok(Page::new(MESSAGE_OK, records, page, per_page, total))
    }
}

impl<T: Query> Query for SortedAndPaginated<T> {
    type SqlType = (T::SqlType, Integer);
}

impl<T> RunQueryDsl<PgConnection> for SortedAndPaginated<T> {}

impl<T> QueryFragment<Pg> for SortedAndPaginated<T>
where
    T: QueryFragment<Pg>,
{
    fn walk_ast<'b>(&'b self, mut out: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<Integer, _>(&self.per_page)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<Integer, _>(&self.offset)?;
        Ok(())
    }
}
