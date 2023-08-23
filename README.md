# Artifacts Repo -- to store and serve build artifacts from Jenkins

This project utilizes the [Diesel ORM](https://diesel.rs), [Actix Web](https://actix.rs) for setting up a backend restful api server, and uses [Angular](https://angular.io) for getting a frontend UI.

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

| Endpoint | HTTP Method | Description |
| :-- | :-: | :-- |
| `api/release` | GET | List all releases. `curl -X GET -i https://dl.senscomm.com/api/release --noproxy '*'`|
| `api/release/new` | POST | Post a new release. |
| `api/release/{id}` | GET | Get a release with the specific id. `curl -X GET -i https://dl.senscomm.com/api/release/1 --noproxy '*'` |
| `api/release/date/{date}` | GET | Get releases with the specific date. `curl -X GET -i https://dl.senscomm.com/api/release/date/2023-08-23 --noproxy '*'` |
| `api/release/repo/date` | POST | Get a release with specified repo name and release date. `curl -X POST -k -H 'Content-Type: application/json' -i 'https://dl.senscomm.com/api/release/repo/date' --data '{"repo": "wise", "date": "2023-08-23"}' --noproxy '*'` |
| `api/repository` | GET | List all `repo`. `curl -X GET -i https://dl.senscomm.com/api/repository --noproxy '*'` |
| `api/release/repository/{repo}` | GET | List all releases under specific repo. `curl -X GET -i https://dl.senscomm.com/api/release/repository/wise --noproxy '*'` |
| `api/artifact/{a_id}` | GET | Get a artifact with it's assiociated release, changelogs and affected files info. `curl -X GET -i https://dl.senscomm.com/api/artifact/1 --noproxy '*'` |
| `api/artifact` | POST | Get a artifact which has specified repo name, release date and defconfig. `curl -X POST -k -H 'Content-Type: application/json' -i 'https://dl.senscomm.com/api/artifact' --data '{"repo": "wise", "date": "2023-08-23", "defconfig": "scm1612bu_defconfig"}' --noproxy '*'` |


## `ReleaseDTO` JSON structure

```json
{
    "release": {
        "org": String,
        "repo": String,
        "release_channel": String,
        "diffs_url": String
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
