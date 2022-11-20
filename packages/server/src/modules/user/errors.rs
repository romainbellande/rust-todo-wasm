use crate::errors::WebError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("could not save user {0}")]
    CouldNotSaveUser(String),

    #[error("user {0} not found")]
    NotFound(String),
}

impl Into<WebError> for Error {
    fn into(self) -> WebError {
        match self.clone() {
            Self::CouldNotSaveUser(_) => WebError {
                code: 1,
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: self.to_string(),
            },
            Self::NotFound(_) => WebError {
                code: 2,
                status: StatusCode::NOT_FOUND,
                message: self.to_string(),
            },
        }
    }
}
