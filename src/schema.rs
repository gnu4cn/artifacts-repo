// @generated automatically by Diesel CLI.

diesel::table! {
    affected_files (id) {
        id -> Int4,
        #[max_length = 10]
        file_edit_type -> Varchar,
        #[max_length = 511]
        file_path -> Varchar,
        repository_id -> Int4,
        release_id -> Int4,
    }
}

diesel::table! {
    artifacts (id) {
        id -> Int4,
        #[max_length = 1023]
        url -> Varchar,
        filesize -> Int8,
        #[max_length = 255]
        build_log_url -> Nullable<Varchar>,
        repository_id -> Int4,
        release_id -> Int4,
        defconfig_id -> Int4,
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
        repository_id -> Int4,
        release_id -> Int4,
    }
}

diesel::table! {
    defconfigs (id) {
        id -> Int4,
        #[max_length = 255]
        config -> Varchar,
        repository_id -> Int4,
    }
}

diesel::table! {
    releases (id) {
        id -> Int4,
        #[max_length = 255]
        release_channel -> Varchar,
        #[max_length = 512]
        diffs_url -> Nullable<Varchar>,
        released_at -> Date,
        repository_id -> Int4,
    }
}

diesel::table! {
    repositories (id) {
        id -> Int4,
        #[max_length = 255]
        org -> Varchar,
        #[max_length = 255]
        repo -> Varchar,
    }
}

diesel::joinable!(affected_files -> releases (release_id));
diesel::joinable!(affected_files -> repositories (repository_id));
diesel::joinable!(artifacts -> defconfigs (defconfig_id));
diesel::joinable!(artifacts -> releases (release_id));
diesel::joinable!(artifacts -> repositories (repository_id));
diesel::joinable!(changelogs -> releases (release_id));
diesel::joinable!(changelogs -> repositories (repository_id));
diesel::joinable!(defconfigs -> repositories (repository_id));
diesel::joinable!(releases -> repositories (repository_id));

diesel::allow_tables_to_appear_in_same_query!(
    affected_files,
    artifacts,
    changelogs,
    defconfigs,
    releases,
    repositories,
);
