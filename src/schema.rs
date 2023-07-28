// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "channel_type"))]
    pub struct ChannelType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "edit_type"))]
    pub struct EditType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "size_unit"))]
    pub struct SizeUnit;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EditType;

    affected_files (affected_file_id) {
        affected_file_id -> Int4,
        file_edit_type -> EditType,
        #[max_length = 511]
        file_path -> Varchar,
        release_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::SizeUnit;

    artifacts (artifact_id) {
        artifact_id -> Int4,
        #[max_length = 255]
        filename -> Varchar,
        filesize -> Numeric,
        filesize_unit -> SizeUnit,
        release_id -> Int4,
    }
}

diesel::table! {
    changelogs (changelog_id) {
        changelog_id -> Int4,
        #[max_length = 7]
        commit_id -> Bpchar,
        commited_at -> Timestamp,
        #[max_length = 2048]
        commit_comment -> Varchar,
        #[max_length = 255]
        commited_by -> Varchar,
        release_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ChannelType;

    releases (release_id) {
        release_id -> Int4,
        channel -> ChannelType,
        #[max_length = 255]
        repo_fullname -> Varchar,
        #[max_length = 512]
        diffs_url -> Varchar,
        released_at -> Date,
    }
}

diesel::joinable!(affected_files -> releases (release_id));
diesel::joinable!(artifacts -> releases (release_id));
diesel::joinable!(changelogs -> releases (release_id));

diesel::allow_tables_to_appear_in_same_query!(
    affected_files,
    artifacts,
    changelogs,
    releases,
);
