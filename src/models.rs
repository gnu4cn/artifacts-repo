use serde::Serialize;

use crate::schema::{releases, changelogs, artifacts, affected_files};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

