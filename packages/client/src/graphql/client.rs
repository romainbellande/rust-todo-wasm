use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/todos.graphql",
)]
pub struct TodosQuery;

impl TodosQuery {
    pub async fn send(variables: todos_query::Variables) -> Result<(), Box<dyn Error>> {
        let body = Self::build_query(variables);

        let client = reqwest::Client::new();

        let res = client.post("http://127.0.0.1:3000/graphql").json(&body).send().await?;

        log::debug!("send");

        let response_body: Response<todos_query::ResponseData> = res.json().await?;

        Ok(())
    }
}

pub type TodosQueryPayload = todos_query::Variables;