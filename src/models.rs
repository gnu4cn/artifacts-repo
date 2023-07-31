use serde::Serialize;

use crate::schema::{releases, changelogs, artifacts, affected_files};

#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::ChannelType"]
pub enum ChannelType {
    Nightly,
    Beta,
    Stable,
}

#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EditType"]
pub enum ChannelType {
    Add,
    Delete,
    Edit,
}

#[derive(diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::SizeUnit"]
pub enum ChannelType {
    Kb,
    Mb,
    Gb,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

