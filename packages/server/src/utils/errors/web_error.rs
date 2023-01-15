use std::fmt;
use async_graphql::{Error as GraphQLError, ErrorExtensions};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;

use super::{AppError, CommonError};

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

// impl From<Box<dyn AppError>> for WebError {
//     fn from(value: Box<dyn AppError>) -> Self {
//         Self {
//             status: value.get_status_code(),
//             code: value.get_code(),
//             message: value.to_string(),
//         }
//     }
// }

impl ErrorExtensions for WebError {
    fn extend(&self) -> GraphQLError {
        GraphQLError::new(self.message.clone()).extend_with(|_, e| {
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

