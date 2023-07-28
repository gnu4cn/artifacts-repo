-- Your SQL goes here
CREATE TYPE size_unit AS ENUM ('kb', 'mb', 'gb');

CREATE TABLE artifacts (
    artifact_id SERIAL PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    filesize NUMERIC(4, 2) NOT NULL,
    filesize_unit size_unit NOT NULL,
    release_id INTEGER NOT NULL REFERENCES releases(release_id)
);
