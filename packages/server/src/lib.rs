mod command;
mod config;
mod db;
mod fixtures;
mod graphql;
mod modules;
mod serve_client;
mod utils;

use async_graphql::{EmptySubscription, Schema};
use axum::{routing::get, Extension, Router, Server};
use config::CONFIG;
use db::Database;
use graphql::{graphiql, graphql_handler, MutationRoot, QueryRoot};
use http::Method;
use migration::{Migrator, MigratorTrait};
use serve_client::serve_client;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub async fn start() {
    let conn = Database::new().get_connection().await;

    let _cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Migrator::up(&conn, None)
        .await
        .expect("could not migrate database");

    fixtures::exec(&conn)
        .await
        .map_err(|err| {
            panic!("an error occured while executing fixtures: {err:?}");
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
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(CorsLayer::permissive())
        .fallback(serve_client(CONFIG.client_dir.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], CONFIG.port));

    println!("GraphiQL IDE: http://127.0.0.1:{}/graphql", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
