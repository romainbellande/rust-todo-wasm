use crate::errors::WebError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("wrong credentials")]
    WrongCredentials,

    #[error("missing credentials")]
    MissingCredentials,

    #[error("an error occured during token creation")]
    TokenCreation,

    #[error("invalid token")]
    InvalidToken,
}

impl Into<WebError> for AuthError {
    fn into(self) -> WebError {
        match self {
            Self::WrongCredentials => WebError {
                code: 1,
                status: StatusCode::UNAUTHORIZED,
                message: self.to_string(),
            },
            Self::MissingCredentials => WebError {
                code: 2,
                status: StatusCode::BAD_REQUEST,
                message: self.to_string(),
            },
            Self::TokenCreation => WebError {
                code: 3,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: self.to_string(),
            },
            Self::InvalidToken => WebError {
                code: 4,
                status: StatusCode::BAD_REQUEST,
                message: self.to_string(),
            },
        }
    }
}
