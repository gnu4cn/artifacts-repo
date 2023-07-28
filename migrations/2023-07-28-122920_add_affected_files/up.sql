-- Your SQL goes here
CREATE TYPE edit_type AS ENUM ('add', 'edit', 'delete');

CREATE TABLE affected_files (
    affected_file_id SERIAL PRIMARY KEY,
    file_edit_type edit_type NOT NULL,
    file_path VARCHAR(511) NOT NULL,
    release_id INTEGER NOT NULL REFERENCES releases(release_id)
);
