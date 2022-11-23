use super::request;
use crate::Error;
use graphql_client::GraphQLQuery;
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
    pub async fn send(variables: LoginPayload) -> Result<login_query::ResponseData, Error> {
        request::<Self>(variables).await
    }
}

pub type LoginPayload = login_query::Variables;
