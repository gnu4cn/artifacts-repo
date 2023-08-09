use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Nightly,
    Beta,
    Stable,
}

#[derive(DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::EditType"]
pub enum EditType {
    Add,
    Edit,
    Delete,
}

#[derive(DbEnum, Debug, Deserialize, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::SizeUnit"]
pub enum SizeUnit {
    Kb,
    Mb,
    Gb,
}
