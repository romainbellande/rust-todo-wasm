# Rust Todo wasm

## Purpose

The purpose of this project is to create a fullstack app thanks to the following tools:

* [yew](https://github.com/yewstack/yew): rust / wasm framework for building client web apps
* [axum](https://github.com/tokio-rs/axum): Ergonomic and modular web framework built with Tokio, Tower, and Hyper
* [edgedb](https://github.com/edgedb/edgedb): A graph-relational database with declarative schema, built-in migration system, and a next-generation query language

Furthermore, I want to libraries around these tools to make the development as easy as possible.

For example:

* Create a ui library thanks to yew and tailwing.
* Ensure data model consistency between client and server thanks to a shared crate.
* Create a browser toolkit crate library for common browser tasks (e.g. data )
* Create a server toolkit crate library for common server tasks

## References

* [rust wasm frontend setup with axum](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/)
* [async graphql axum example](https://github.com/async-graphql/examples/tree/master/axum/subscription)
