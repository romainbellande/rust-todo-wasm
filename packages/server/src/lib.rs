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
use axum_extra::routing::SpaRouter;
use config::{RustEnv, CONFIG};
use db::Database;
use graphql::{graphiql, graphql_handler, MutationRoot, QueryRoot};
use http::{Method, HeaderValue, header};
use migration::{Migrator, MigratorTrait};

use std::fs::File;
use std::{io::Write, net::SocketAddr};
use tower_http::cors::CorsLayer;

pub async fn start() {
    let conn = Database::new().get_connection().await;

    let cors = CorsLayer::new()
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)
        // allow `GET` and `POST` when accessing the resource
        .allow_methods(vec![Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin("http://dev.localhost.com:8080".parse::<HeaderValue>().unwrap());

    Migrator::up(&conn, None)
        .await
        .expect("could not migrate database");

    println!("migration succeed");

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

    if CONFIG.rust_env.clone() == RustEnv::Development {
        let mut file = File::create("../client/graphql/schema.graphql")
            .expect("an error occured while writing graphql schema file");

        file.write_all(schema.sdl().as_bytes())
            .expect("error while writing graphql schema content");

        println!("graphql schema writing succeed");
    }

    let spa = SpaRouter::new("/assets", CONFIG.client_dir.clone());

    let app = Router::new()
        .merge(spa)
        .route("/graphql", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.port));

    println!("GraphiQL IDE: http://0.0.0.0:{}/graphql", CONFIG.port);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
