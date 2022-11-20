use crate::Error;
use crate::CONFIG;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::fmt::Debug;

pub async fn request<Q>(variables: Q::Variables) -> Result<Q::ResponseData, Error>
where
    Q: GraphQLQuery + Clone + Debug,
    <Q as GraphQLQuery>::ResponseData: Clone + Debug,
{
    let client = reqwest::Client::new();

    let base_url = if let Some(api_url) = CONFIG.api_url.clone() {
        api_url
    } else {
        web_sys::window().unwrap().location().origin().unwrap()
    };

    let url = format!("{}/graphql", base_url);

    let body = Q::build_query(variables);

    let request_builder = client
        .post(url)
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
