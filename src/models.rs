use serde::Serialize;
use diesel_derive_enum::DbEnum;

use crate::schema::{releases, changelogs, artifacts, affected_files};

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Nightly,
    Beta,
    Stable,
}

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::EditType"]
pub enum EditType {
    Add,
    Edit,
    Delete,
}

#[derive(DbEnum, Debug)]
#[ExistingTypePath = "crate::schema::sql_types::SizeUnit"]
pub enum SizeUnit {
    Kb,
    Mb,
    Gb,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

