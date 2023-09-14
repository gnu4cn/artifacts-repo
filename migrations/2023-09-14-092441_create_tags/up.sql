-- Create table tags;
CREATE TABLE tags (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    release_id SERIAL NOT NULL REFERENCES releases(id),
    repository_id SERIAL NOT NULL REFERENCES repositories(id)
);

