# Rust Todo wasm

## Purpose

The purpose of this project is to create a fullstack app thanks to the following tools:

* [yew](https://github.com/yewstack/yew): Rust / Wasm framework for building client web apps
* [axum](https://github.com/tokio-rs/axum): Ergonomic and modular web framework built with Tokio, Tower, and Hyper
* [seaorm](https://www.sea-ql.org/SeaORM/): A relational ORM to help you build web services in Rustquery language

Furthermore, I want to libraries around these tools to make the development as easy as possible.

For example:

* Create a ui library thanks to yew and tailwind.
* Ensure data model consistency between client and server thanks to a shared crate.
* Create a browser toolkit crate library for common browser tasks (e.g. data )
* Create a server toolkit crate library for common server tasks

## References

* [rust wasm frontend setup with axum](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/)
* [async graphql axum example](https://github.com/async-graphql/examples/tree/master/axum/subscription)
