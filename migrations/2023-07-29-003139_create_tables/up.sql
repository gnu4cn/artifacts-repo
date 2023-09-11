-- Create table repositories;
CREATE TABLE repositories (
    id SERIAL PRIMARY KEY NOT NULL,
    org VARCHAR(255) NOT NULL,
    repo VARCHAR(255) NOT NULL
);

-- Create table releases;
CREATE TABLE releases (
    id SERIAL PRIMARY KEY NOT NULL,
    release_channel VARCHAR(255) NOT NULL,
    diffs_url VARCHAR(512),
    released_at DATE NOT NULL DEFAULT CURRENT_DATE,
    repository_id SERIAL NOT NULL REFERENCES repositories(id)
);

-- Create table tags;
CREATE TABLE tags (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    commit_id VARCHAR(48) NOT NULL,
    release_id SERIAL NOT NULL REFERENCES releases(id),
    repository_id SERIAL NOT NULL REFERENCES repositories(id)
);

-- Create table changelogs;
CREATE TABLE changelogs (
    id SERIAL PRIMARY KEY NOT NULL,
    commit_id CHAR(7) NOT NULL,
    commit_comment VARCHAR(2048) NOT NULL,
    commited_by VARCHAR(255) NOT NULL,
    release_id SERIAL NOT NULL REFERENCES releases(id)
);


-- Create table artifacts;
CREATE TABLE artifacts (
    id SERIAL PRIMARY KEY NOT NULL,
    defconfig VARCHAR(255) NOT NULL,
    url VARCHAR(1023) NOT NULL,
    filesize BIGINT NOT NULL,
    build_log_url VARCHAR(255),
    repository_id SERIAL NOT NULL REFERENCES repositories(id),
    release_id SERIAL NOT NULL REFERENCES releases(id)
);


-- Create table affected_files;
CREATE TABLE affected_files (
    id SERIAL PRIMARY KEY NOT NULL,
    file_edit_type VARCHAR(10) NOT NULL,
    file_path VARCHAR(511) NOT NULL,
    release_id SERIAL NOT NULL REFERENCES releases(id)
);
