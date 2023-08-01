use diesel::{prelude::*, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    config::db::Connection,
    schema::releases::{self, dsl::*},
    models::db_enum::ChannelType,
};
