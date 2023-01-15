use axum::http::StatusCode;
use thiserror::Error;
use crate::utils::errors::{AppError, WebError};

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
            Self::NotFound(_) => "USER_NOT_FOUND",
            Self::CouldNotSaveUser(_) => "COULD_NOT_SAVE_USER"
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

impl Into<WebError> for Error {
    fn into(self) -> WebError {
        match self.clone() {
            Self::CouldNotSaveUser(_) => WebError {
                code: self.get_code(),
                status: self.get_status_code(),
                message: self.to_string(),
            },
            Self::NotFound(_) => WebError {
                code: self.get_code(),
                status: self.get_status_code(),
                message: self.to_string(),
            },
        }
    }
}
