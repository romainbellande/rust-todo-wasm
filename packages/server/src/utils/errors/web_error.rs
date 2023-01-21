use async_graphql::{Error as GraphQLError, ErrorExtensions};
use axum::response::{IntoResponse, Response};
use http::StatusCode;

use std::fmt;

use super::AppError;

#[derive(Debug, Clone)]
pub struct WebError {
    pub code: String,

    pub status: StatusCode,

    pub message: String,
}

impl fmt::Display for WebError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl WebError {
    pub fn new(status: StatusCode, message: String, code: impl AppError) -> Self {
        Self {
            status,
            message,
            code: code.to_string(),
        }
    }
}

impl ErrorExtensions for WebError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(self.message.clone()).extend_with(|_, e| {
            println!("{}", self.code.clone());
            e.set("code", self.code.clone());
            e.set("status", self.status.as_u16());
        })
    }
}

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (self.status, self).into_response()
    }
}

macro_rules! web_error {
    ($t:ty) => {
        impl Into<WebError> for $t {
            fn into(self) -> WebError {
                WebError {
                    code: self.get_code(),
                    status: self.get_status_code(),
                    message: self.to_string(),
                }
            }
        }
    };
}

pub(crate) use web_error;
