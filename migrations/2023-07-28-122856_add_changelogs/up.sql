-- Your SQL goes here
CREATE TABLE changelogs (
    changelog_id SERIAL PRIMARY KEY,
    commit_id CHAR(7) NOT NULL,
    commited_at TIMESTAMP NOT NULL,
    commit_comment VARCHAR(2048) NOT NULL,
    commited_by VARCHAR(255) NOT NULL,
    release_id INTEGER NOT NULL REFERENCES releases(release_id)
);
