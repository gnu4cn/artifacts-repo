// @generated automatically by Diesel CLI.

diesel::table! {
    affected_files (id) {
        id -> Int4,
        #[max_length = 10]
        file_edit_type -> Varchar,
        #[max_length = 511]
        file_path -> Varchar,
        release_id -> Int4,
    }
}

diesel::table! {
    artifacts (id) {
        id -> Int4,
        #[max_length = 255]
        filename -> Varchar,
        filesize -> Int8,
        release_id -> Int4,
    }
}

diesel::table! {
    changelogs (id) {
        id -> Int4,
        #[max_length = 7]
        commit_id -> Bpchar,
        #[max_length = 2048]
        commit_comment -> Varchar,
        #[max_length = 255]
        commited_by -> Varchar,
        release_id -> Int4,
    }
}

diesel::table! {
    releases (id) {
        id -> Int4,
        #[max_length = 255]
        repo_fullname -> Varchar,
        #[max_length = 512]
        diffs_url -> Nullable<Varchar>,
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
