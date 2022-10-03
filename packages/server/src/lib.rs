mod command;
mod config;
mod graphql;
mod modules;

use async_graphql::{EmptySubscription, Schema};
use axum::{routing::get, Extension, Router, Server};
use config::CONFIG;
use graphql::{graphiql, graphql_handler, MutationRoot, QueryRoot};
use std::net::SocketAddr;

pub async fn start() {
    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));

    println!("GraphiQL IDE: http://127.0.0.1:{}", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
