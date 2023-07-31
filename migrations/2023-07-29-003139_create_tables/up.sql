-- Create table releases;
CREATE TYPE channel_type AS ENUM ('nightly', 'beta', 'stable');

CREATE TABLE releases (
    release_id VARCHAR(255) PRIMARY KEY,
    channel channel_type NOT NULL DEFAULT 'nightly',
    repo_fullname VARCHAR(255) NOT NULL,
    diffs_url VARCHAR(512),
    released_at DATE NOT NULL DEFAULT CURRENT_DATE
);

-- Create table changelogs;
CREATE TABLE changelogs (
    changelog_id VARCHAR(255) PRIMARY KEY,
    commit_id CHAR(7) NOT NULL,
    commited_at TIMESTAMP NOT NULL,
    commit_comment VARCHAR(2048) NOT NULL,
    commited_by VARCHAR(255) NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(release_id)
);


-- Create table artifacts;
CREATE TYPE size_unit AS ENUM ('kb', 'mb', 'gb');

CREATE TABLE artifacts (
    artifact_id VARCHAR(255) PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    filesize NUMERIC(4, 2) NOT NULL,
    filesize_unit size_unit NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(release_id)
);


-- Create table affected_files;
CREATE TYPE edit_type AS ENUM ('add', 'edit', 'delete');

CREATE TABLE affected_files (
    affected_file_id VARCHAR(255) PRIMARY KEY,
    file_edit_type edit_type NOT NULL,
    file_path VARCHAR(511) NOT NULL,
    release_id VARCHAR(255) NOT NULL REFERENCES releases(release_id)
);
