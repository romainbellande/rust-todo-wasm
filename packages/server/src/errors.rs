use std::fmt;

use async_graphql::{Error as GraphQLError, ErrorExtensions};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct WebError {
    pub code: u16,

    pub status: StatusCode,

    pub message: String,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl WebError {
    pub fn new(status: StatusCode, message: String, code: Option<u16>) -> Self {
        let code = if let Some(code) = code {
            code
        } else {
            status.as_u16()
        };

        Self {
            status,
            message,
            code,
        }
    }
}

impl ErrorExtensions for WebError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(format!("{}", self.message)).extend_with(|err, e| {
            e.set("code", self.code);
            e.set("status", self.status.as_u16());
        })
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (self.status, self).into_response()
    }
}

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("internal server error: {0}")]
    InternalServerError(String),
}

impl Into<WebError> for Error {
    fn into(self) -> WebError {
        match self.clone() {
            Self::InternalServerError(_) => WebError {
                message: self.to_string(),
                status: StatusCode::INTERNAL_SERVER_ERROR,
                code: 1,
            },
        }
    }
}

macro_rules! graphql_error {
    ($e:expr) => {{
        let web_error: WebError = $e.into();
        let graphql_error: async_graphql::Error = web_error.into();
        graphql_error
    }};
}

pub(crate) use graphql_error;
