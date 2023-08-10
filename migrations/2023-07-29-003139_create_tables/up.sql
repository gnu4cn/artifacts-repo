-- Create table releases;
CREATE TYPE channel_type AS ENUM ('nightly', 'beta', 'stable');

CREATE TABLE releases (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    channel channel_type NOT NULL DEFAULT 'nightly',
    repo_fullname VARCHAR(255) NOT NULL,
    diffs_url VARCHAR(512),
    released_at DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Create table changelogs;
CREATE TABLE changelogs (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    commit_id CHAR(7) NOT NULL,
    commited_at TIMESTAMP NOT NULL,
    commit_comment VARCHAR(2048) NOT NULL,
    commited_by VARCHAR(255) NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(id)
);


-- Create table artifacts;
CREATE TYPE size_unit AS ENUM ('kb', 'mb', 'gb');

CREATE TABLE artifacts (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    filesize NUMERIC(4, 2) NOT NULL,
    filesize_unit size_unit NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(id)
);


-- Create table affected_files;
CREATE TYPE edit_type AS ENUM ('add', 'edit', 'delete');

CREATE TABLE affected_files (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    file_edit_type edit_type NOT NULL,
    file_path VARCHAR(511) NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(id)
);
