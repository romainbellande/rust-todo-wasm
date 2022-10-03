mod command;
mod config;
mod graphql;
mod models;

use axum::{Router, Server};

use std::net::SocketAddr;

pub async fn start() {
    let app = Router::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
