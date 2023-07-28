-- Your SQL goes here
CREATE TYPE channel_type AS ENUM ('nightly', 'beta', 'stable');

CREATE TABLE releases (
    release_id SERIAL PRIMARY KEY,
    channel channel_type NOT NULL DEFAULT 'nightly',
    repo_fullname VARCHAR(255) NOT NULL,
    diffs_url VARCHAR(512) NOT NULL,
    released_at DATE NOT NULL DEFAULT CURRENT_DATE
);
