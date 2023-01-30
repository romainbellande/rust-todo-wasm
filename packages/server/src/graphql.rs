use crate::config::CONFIG;
use crate::modules::auth::{
    extractor::{get_access_token_from_headers, get_refresh_token_from_headers},
    AuthQuery,
};
use crate::modules::todo::{TodoMutation, TodoQuery};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::header::HeaderMap,
    response::{Html, IntoResponse},
    Extension,
};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TodoQuery, AuthQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(TodoMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphql_handler(
    Extension(schema): Extension<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(access_token) = get_access_token_from_headers(&headers) {
        req = req.data(access_token);
    }

    if let Some(refresh_token) = get_refresh_token_from_headers(&headers) {
        req = req.data(refresh_token);
    }

    schema.execute(req).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("http://0.0.0.0:{}/graphql", CONFIG.port);

    Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}
