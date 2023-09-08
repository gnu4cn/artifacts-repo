# Artifacts Repo -- to store and serve build artifacts from Jenkins

This project utilizes the [Diesel ORM](https://diesel.rs) backed by [PostgreSQL](https://www.postgresql.org/), [Actix Web](https://actix.rs) for setting up a backend restful api server, and uses [Angular](https://angular.io) for getting a frontend UI. This project has taken inspiration from [SakaDream/actix-web-rest-api-with-jwt](https://github.com/SakaDream/actix-web-rest-api-with-jwt).

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

| Endpoint | Description | Usage |
| :-- | :-- | :-- |
| `api/release/new` | Post a new release. | |
| `api/release` | List all releases. | `curl -X GET -i https://HOST/api/release --noproxy '*'`|
| `api/release/repository` | List all releases under specific repo. | `curl -X POST -H 'Content-Type: application/json' -i http://HOST/api/release/repository --data '{"org":"Senscomm","repo":"taihu_wise"}' --noproxy '*'` |
| `api/release/days` | List all days release available. | `curl -X GET -k -i 'https://HOST/api/release/days' --noproxy '*'` |
| `api/release/repo/date` | Get a release with specified repo name and release date. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/release/repo/date' --data '{"repo":{"org": "Senscomm", "repo": "wise"}, "date": "2023-08-29"}' --noproxy '*'` |
| `api/release/date/{date}` | Get releases with the specific date. | `curl -X GET -i https://HOST/api/release/date/2023-08-23 --noproxy '*'` |
| `api/release/{id}` | Get a release with the specific id. | `curl -X GET -i https://HOST/api/release/1 --noproxy '*'` |
| `api/repository` | List all `repo`. | `curl -X GET -i https://HOST/api/repository --noproxy '*'` |
| `api/repository/brief` | List all `repo`'s brief data. | `curl -X GET -k -i 'https://HOST/api/repository/brief' --noproxy '*'` |
| `api/repository/brief/{repo_id}` | Fetch specific `repo`'s brief data. | `curl -X GET -k -i 'https://HOST/api/repository/brief/1' --noproxy '*'` |
| `api/repository/defconfig` | List all defconfigs under specific repo. | `curl -X POST -H 'Content-Type: application/json' -i http://HOST/api/repository/defconfig --data '{"org":"Senscomm","repo":"wise"}' --noproxy '*'` |
| `api/artifact/{a_id}` | Get a artifact with it's assiociated release, changelogs and affected files info. | `curl -X GET -i https://HOST/api/artifact/2 --noproxy '*'` |
| `api/artifact` | Get a artifact which has specified repo name, release date and defconfig. | `curl -X POST -k -H 'Content-Type: application/json' -i 'https://HOST/api/artifact' --data '{"repo":{"org": "Senscomm", "repo": "wise"}, "date": "2023-08-29", "defconfig": "scm1612_ate_defconfig"}' --noproxy '*'` |


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
