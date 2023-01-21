use super::request;
use graphql_client::GraphQLQuery;
use shared::errors::AppError;
use std::fmt::Debug;

#[derive(GraphQLQuery, Clone, Debug)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/todos.graphql",
    response_derives = "Debug,Clone",
    variables_derives = "Debug,Clone"
)]
pub struct FindTodos;

impl FindTodos {
    pub async fn send(variables: FindTodosPayload) -> Result<find_todos::ResponseData, AppError> {
        request::<Self>(variables).await
    }
}

pub type FindTodosPayload = find_todos::Variables;
