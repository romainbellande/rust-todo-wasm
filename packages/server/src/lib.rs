mod command;
mod config;
mod db;
mod fixtures;
mod graphql;
mod modules;
mod utils;

use async_graphql::{EmptySubscription, Schema};
use axum::{routing::get, Extension, Router, Server};
use config::CONFIG;
use db::Database;
use graphql::{graphiql, graphql_handler, MutationRoot, QueryRoot};
use migration::{Migrator, MigratorTrait};
use std::net::SocketAddr;

pub async fn start() {
    let conn = Database::new().get_connection().await;

    Migrator::up(&conn, None)
        .await
        .expect("could not migrate database");

    fixtures::exec(&conn)
        .await
        .map_err(|err| {
            panic!("an error occured while executing fixtures: {:?}", err);
        })
        .unwrap();

    let schema = Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription,
    )
    .data(conn)
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
