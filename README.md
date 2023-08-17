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
| `api/release` | GET | List all releases |
| `api/release/new` | POST | Post a new release |
| `api/release/{id}` | GET | Get a release with the specific id |
| `api/release/date/{date}` | GET | Get released with the specific date(`2023-08-17`) |
| `api/repository` | GET | List all repo_fullname |
| `api/release/repository/{repo}` | GET | List all releases under specific repo. |
