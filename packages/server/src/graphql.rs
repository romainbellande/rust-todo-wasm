use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
    http::header::HeaderMap,
};
use crate::config::CONFIG;
use crate::modules::auth::AuthQuery;
use crate::modules::todo::{TodoMutation, TodoQuery};

#[derive(MergedObject, Default)]
pub struct QueryRoot(TodoQuery, AuthQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(TodoMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug)]
pub struct AccessToken(pub String);

fn get_token_from_headers(headers: &HeaderMap) -> Option<AccessToken> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| AccessToken(s.to_string())).ok())
}

pub async fn graphql_handler(Extension(schema): Extension<AppSchema>, headers: HeaderMap, req: GraphQLRequest) -> GraphQLResponse {
    let mut req = req.into_inner();

    if let Some(token) = get_token_from_headers(&headers) {
        req = req.data(token);
    }

    schema.execute(req).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    let endpoint = format!("http://0.0.0.0:{}/graphql", CONFIG.port);

    Html(GraphiQLSource::build().endpoint(&endpoint).finish())
}
