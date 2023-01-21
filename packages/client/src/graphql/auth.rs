use super::request;
use graphql_client::GraphQLQuery;
use shared::errors::AppError;
use std::fmt::Debug;

#[derive(GraphQLQuery, Clone, Debug)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/login.graphql",
    response_derives = "Debug,Clone",
    variables_derives = "Debug,Clone"
)]
pub struct LoginQuery;

impl LoginQuery {
    pub async fn send(variables: LoginPayload) -> Result<login_query::ResponseData, AppError> {
        request::<Self>(variables).await
    }
}

pub type LoginPayload = login_query::Variables;
