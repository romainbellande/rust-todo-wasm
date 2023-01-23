use crate::utils::errors::{web_error, AppError, WebError};
use axum::http::StatusCode;
use shared::errors;
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

    #[error("refresh token missing")]
    MissingRefreshToken,

    #[error("access token missing")]
    MissingAccessToken
}

impl AppError for AuthError {
    fn get_code(&self) -> String {
        let code = match self {
            Self::MissingCredentials => errors::AppError::MissingCredentials,
            Self::InvalidToken => errors::AppError::InvalidToken,
            Self::TokenCreation => errors::AppError::TokenCreation,
            Self::WrongCredentials => errors::AppError::WrongCredentials,
            Self::MissingRefreshToken => errors::AppError::MissingRefreshToken,
            Self::MissingAccessToken => errors::AppError::MissingAccessToken,
        };

        code.to_string()
    }

    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::WrongCredentials => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::BAD_REQUEST,
            Self::MissingCredentials => StatusCode::BAD_REQUEST,
            Self::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MissingRefreshToken => StatusCode::FORBIDDEN,
            Self::MissingAccessToken => StatusCode::FORBIDDEN,
        }
    }
}

web_error!(AuthError);
