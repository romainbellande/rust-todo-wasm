use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};

use crate::config::CONFIG;
use crate::modules::todo::{TodoMutation, TodoQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TodoQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(TodoMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("http://0.0.0.0:{}/graphql", CONFIG.port);

    Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}
