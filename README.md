![build](https://github.com/romainbellande/rust-todo-wasm/actions/workflows/build.yml/badge.svg) ![tests](https://github.com/romainbellande/rust-todo-wasm/actions/workflows/tests.yml/badge.svg)

# Rust Todo wasm

## Purpose

The purpose of this project is to create a fullstack app thanks to the following tools:

* [yew](https://github.com/yewstack/yew): Rust / Wasm framework for building client web apps
* [axum](https://github.com/tokio-rs/axum): Ergonomic and modular web framework built with Tokio, Tower, and Hyper
* [seaorm](https://www.sea-ql.org/SeaORM/): A relational ORM to help you build web services in Rustquery language
* [tailwind](https://tailwindcss.com): A utility-first CSS framework.
* [async graphql](https://github.com/async-graphql/async-graphql): A utility-first CSS framework.

## Requirements

* docker and docker-compose
* Rustc / cargo installed
* Install npm dependencies thanks to `cd packages/client && npm i`
* [Install taskfile](https://taskfile.dev/installation/)
* Initialize the project thanks to `task init` command.

### Environment variables

During development all environment variables are already set in [.env file](./.env).

| **name**       | **description**                                                                                                                                       | **required** | **example**                                            |
|----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|--------------|--------------------------------------------------------|
| DATABASE_URL   | Postgres connection string.                                                                                                                           | true         | postgres://pguser:pgpass@127.0.0.1:5449/rust_todo_wasm |
| RUST_LOG       | Debug level.                                                                                                                                          | false        | debug                                                  |
| JWT_SECRET     | Jwt secret to generate jwt.                                                                                                                           | true         | must be a strong password like string                  |
| SALT           | Used to hash users password. 16 bytes recommended for password hashing. It must be long enough or the application will not start for security reason. | true         | strong password >= 16 bytes                            |
| API_URL        | Must be "http://127.0.0.1:3000" during development and not for the deployed application.                                                              | false        |                                                        |
| ADMIN_EMAIL    | Your email.                                                                                                                                           | true         | your email                                             |
| ADMIN_PASSWORD | Your password. Will be hashed by argon2.                                                                                                              | true         | a new randomly generated password                      |
| RUST_ENV       | Must be set to "development" during development and "production" for the deployed application.                                                        | true         | development                                            |
| PORT           | Server port.                                                                                                                                          | true         | 3000                                                   |

## Start the applications

* run `task run:dev`, server will be running on [http://127.0.0.1:3000](http://127.0.0.1:3000) and client will be running on [http://127.0.0.1:8080](http://127.0.0.1:8080) 

## Resources

* [rust wasm frontend setup with axum](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/)
* [async graphql axum example](https://github.com/async-graphql/examples/tree/master/axum/subscription)
* [axum yew setup](https://github.com/rksm/axum-yew-setup)
