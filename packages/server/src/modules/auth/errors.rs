use crate::utils::errors::{WebError, AppError};
use axum::{
    http::StatusCode,
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

impl AppError for AuthError {
    fn get_code(&self) -> String {
        let code = match self {
            Self::MissingCredentials => "MISSING_CREDENTIALS",
            Self::InvalidToken => "INVALID_TOKEN",
            Self::TokenCreation => "TOKEN_CREATION",
            Self::WrongCredentials => "WRONG_CREDENTIALS",
        };

        code.to_string()
    }

    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::WrongCredentials => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::MissingCredentials => StatusCode::BAD_REQUEST,
            Self::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Into<WebError> for AuthError {
    fn into(self) -> WebError {
        WebError {
            code: self.get_code(),
            status: self.get_status_code(),
            message: self.to_string(),
        }
    }
}
