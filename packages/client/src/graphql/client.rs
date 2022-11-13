use super::request;
use crate::Error;
use graphql_client::GraphQLQuery;
use std::fmt::Debug;

#[derive(GraphQLQuery, Clone, Debug)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/todos.graphql",
    response_derives = "Debug,Clone",
    variables_derives = "Debug,Clone"
)]
pub struct TodosQuery;

impl TodosQuery {
    pub async fn send(variables: TodosQueryPayload) -> Result<todos_query::ResponseData, Error> {
        request::<Self>(variables).await
    }
}

pub type TodosQueryPayload = todos_query::Variables;
