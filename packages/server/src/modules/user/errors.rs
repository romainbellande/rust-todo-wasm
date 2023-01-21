use crate::utils::errors::{web_error, AppError, WebError};
use axum::http::StatusCode;
use shared::errors;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("could not save user {0}")]
    CouldNotSaveUser(String),

    #[error("user {0} not found")]
    NotFound(String),
}

impl AppError for Error {
    fn get_code(&self) -> String {
        let str = match self {
            Self::NotFound(_) => errors::AppError::UserNotFound,
            Self::CouldNotSaveUser(_) => errors::AppError::CouldNotSaveUser,
        };

        str.to_string()
    }

    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::CouldNotSaveUser(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

web_error!(Error);
