use crate::Error;
use graphql_client::{reqwest::post_graphql, GraphQLQuery, QueryBody, Response};
use reqwest::{self, IntoUrl};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn request<Q>(variables: Q::Variables) -> Result<Q::ResponseData, Error>
where
    Q: GraphQLQuery + Clone + Debug,
    <Q as GraphQLQuery>::ResponseData: Clone + Debug,
{
    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:3000/graphql";

    let body = Q::build_query(variables);

    let request_builder = client
        .post("http://127.0.0.1:3000/graphql")
        .header("Content-Type", "application/json")
        .json(&body);

    // let response = post_graphql::<B, _>(&client, url, body).await;

    let response = request_builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<Response<Q::ResponseData>, _> = data.json().await;

            if let Ok(data) = data {
                log::debug!("Response: {:#?}", data);
                if let Some(data) = data.data {
                    Ok(data)
                } else {
                    Err(Error::NotFound)
                }
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => Err(Error::UnprocessableEntity),
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}
