use crate::CONFIG;
use graphql_client::{GraphQLQuery, Response};
use shared::errors::AppError;
use std::fmt::Debug;
use std::str::FromStr;

pub async fn request<Q>(variables: Q::Variables) -> Result<Q::ResponseData, AppError>
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

    let response = request_builder.send().await;

    let data = response.map_err(|_| AppError::RequestError)?;

    if !data.status().is_success() {
        return match data.status().as_u16() {
            401 => Err(AppError::Unauthorized),
            403 => Err(AppError::Forbidden),
            404 => Err(AppError::NotFound),
            500 => Err(AppError::InternalServerError),
            422 => Err(AppError::UnprocessableEntity),
            _ => Err(AppError::RequestError),
        };
    }

    let data: Response<Q::ResponseData> =
        data.json().await.map_err(|_| AppError::DeserializeError)?;
    info!("{:?}", data);

    if let Some(sub_data) = data.data {
        Ok(sub_data)
    } else {
        let errors = data.errors.ok_or(AppError::NoErrorExtension)?;
        let error = errors.get(0).ok_or(AppError::NoErrorExtension)?.clone();
        let extensions = error.extensions.ok_or(AppError::NoErrorExtension)?;
        let err_code = extensions
            .get("code")
            .ok_or(AppError::NoErrorCodeProvided)?;
        let err_str = err_code.as_str().ok_or(AppError::ErrorCodeIsNotStr)?;
        let err = AppError::from_str(err_str).map_err(|_| AppError::UnknownErrorCode)?;
        Err(err)
    }
}
