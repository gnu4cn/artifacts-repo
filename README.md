# Artifacts Repo -- to store and serve build artifacts from Jenkins

This project utilizes the [Diesel ORM](https://diesel.rs) backed by [PostgreSQL](https://www.postgresql.org/), [Actix Web](https://actix.rs) for setting up a backend restful api server, and uses [Angular](https://angular.io) for getting a frontend UI, which located at [gnu4cn/artifacts-repo-spa](https://github.com/gnu4cn/artifacts-repo-spa). This project has taken inspiration from [SakaDream/actix-web-rest-api-with-jwt](https://github.com/SakaDream/actix-web-rest-api-with-jwt).

![Project diagram](./artifacts-repo.png)

*Chart: `artfacts-repo` project diagram*

![Database tables design](./artifacts-repo-db-tables-design.png)

*Chart: `artfacts-repo` project database tables design*


## TODO list

- Implement the pagination feature.

## Prerequisites

Clone this project to your local machine.

```bash
git clone https://github.com/gnu4cn/artifacts-repo.git
```

To run this project, you should have Rust `cargo`, PostgreSQL installed. And creating a user `jenkins` with password `jenkins` in PostgreSQL, then create a database `jenkins`, and grant all its' privileges to user `jenkins`. Or you can create a file `.env` then set the corresponding database name and user credentials in that file.

```env
DATABASE_URL=postgresql://jenkins:jenkins@localhost:5432/jenkins
```

When all things prepared, simply run the following command to start the server.

```console
cd artifacts-repo
cargo run
```

## API list

### `api/release`


| Endpoint | Description | Usage |
| :-- | :-- | :-- |
| `api/release/new` | Post a new release. | |
| `api/release` | List all releases. | `curl -X GET -i https://HOST/api/release --noproxy '*'`|
| `api/release/today` | List releases of today. | `curl -X GET -i https://HOST/api/release/today --noproxy '*'`|
| `api/release/repository` | List all releases under specific repo. | `curl -X POST -H 'Content-Type: application/json' -i https://HOST/api/release/repository --data '{"org":"Senscomm","repo":"taihu_wise"}' --noproxy '*'` |
| `api/release/days` | List all days release available. | `curl -X GET -k -i 'https://HOST/api/release/days' --noproxy '*'` |
| `api/release/repo/date` | Fetch a release with specified repo name and release date. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/release/repo/date' --data '{"repo":{"org": "Senscomm", "repo": "wise"}, "date": "2023-08-29"}' --noproxy '*'` |
| `api/release/date/{date}` | Fetch releases with the specific date. | `curl -X GET -i https://HOST/api/release/date/2023-08-23 --noproxy '*'` |
| `api/release/{id}` | Fetch a release with the specific id. | `curl -X GET -i https://HOST/api/release/1 --noproxy '*'` |


### `api/repository`

| Endpoint | Description | Usage |
| :-- | :-- | :-- |
| `api/repository` | List all `repo`. | `curl -X GET -i https://HOST/api/repository --noproxy '*'` |
| `api/repository/brief` | List all `repo`'s brief data. | `curl -X GET -k -i 'https://HOST/api/repository/brief' --noproxy '*'` |
| `api/repository/brief/{repo_id}` | Fetch specific `repo`'s brief data. | `curl -X GET -k -i 'https://HOST/api/repository/brief/1' --noproxy '*'` |
| `api/repository/defconfig` | List all defconfigs under specific repo. | `curl -X POST -H 'Content-Type: application/json' -i https://HOST/api/repository/defconfig --data '{"org":"Senscomm","repo":"wise"}' --noproxy '*'` |
| `api/repository/tagged` | List all tagged released under specific repo. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/repository/tagged' --data '{"org":"Senscomm","repo":"wise"}' --noproxy '*'` |
| `api/repository/tag/release` | Fetch a release which has the specicified tag under a repository. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/repository/tag/release' --data '{"repo": {"org":"Senscomm","repo":"wise"}, "tag": "WISE-SDK-1.0.1"}' --noproxy '*'` |

### `api/artifact`

| Endpoint | Description | Usage |
| :-- | :-- | :-- |
| `api/artifact/{a_id}` | Fetch a artifact with it's assiociated release, changelogs and affected files info. | `curl -X GET -i https://HOST/api/artifact/2 --noproxy '*'` |
| `api/artifact` | Fetch a artifact which has specified repo name, release date and defconfig. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/artifact' --data '{"repo":{"org": "Senscomm", "repo": "wise"}, "date": "2023-08-29", "defconfig": "scm1612_ate_defconfig"}' --noproxy '*'` |


## `ReleaseDTO` JSON structure

```json
{
    "repo": {
        "org": String,
        "repo": String
    },
    "release":{
        "release_channel": String,
        "diffs_url": String,
        "repository_id": 0
    },
    "tag":{
        "name": String,
        "repository_id": 0,
        "release_id": 0
    },
    "changelogs": [
        {
            "commit_id": String,
            "commit_comment": String,
            "commited_by": String,
            "release_id": 0
        }
    ],
    "artifacts": [
        {
            "defconfig": String,
            "url": String,
            "filesize": i32,
            "build_log_url": String,
            "repository_id": 0,
            "release_id": 0
        }
    ],
    "affected_files": [
        {
            "file_edit_type": String
            "file_path": String,
            "release_id": 0
        }
    ]
}
```

Use this JSON structure to POST a new release.

## How to migrate the database tables

- Run `diesel migrate generate create_tags` command to create migration SQL files under the project's `migrations` directory. The `create_tags` CLI flag is a comment-like command parameter.

- Run `vim migrations/2023-09-14-092441_create_tags/up.sql` to edit both the `up.sql` and `down.sql` files, to add or remove any database table.

- Then run `diesel migrate run` to complete the migration.

> Note:
>
> When use the diesel-orm migration feat, we should use `pg_dump`'s `-a`, `--only-data` option, to only dump the data. And for `diesel migrate run` command, there is a `--all` option for specifying running all the migrations.


## About `git tag`

+ Add a tag and push it to remote

    - `git tag TAG_NAME`

    - `git tag TAG_NAME -a -m "message"`, Create an “annotated” tag with the given message (instead of prompting)

    - `git push origin TAG_NAME`

+ Delete local and remote tag

    - `git tag -d TAG_NAME`

    - `git push --delete origin TAG_NAME`

- List tags by commit date: `git tag --sort=committerdate`
