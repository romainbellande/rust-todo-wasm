use super::WebError;
use http::StatusCode;
use thiserror::Error as ThisError;

pub trait AppError: ToString {
    fn get_code(&self) -> String;

    fn get_status_code(&self) -> StatusCode;
}

#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum CommonError {
    #[error("a bad request occured: {0}")]
    BadRequest(String),

    #[error("an internal error occurred: {0}")]
    InternalServerError(String),

    #[error("insufficient rights to resource")]
    Forbidden,

    #[error("access to the target resource is no longer available at the origin server and that this condition is likely to be permanent")]
    Gone,

    #[error("an error occurred")]
    GenericError,

    #[error("request timeout")]
    RequestTimeout,

    #[error("unauthorized request for user {0}")]
    Unauthorized(String),
}

impl AppError for CommonError {
    fn get_code(&self) -> String {
        let str = match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            Self::Forbidden => "FORBIDDEN",
            Self::Gone => "GONE",
            Self::GenericError => "GENERIC_ERROR",
            Self::RequestTimeout => "REQUEST_TIMEOUT",
            Self::Unauthorized(_) => "UNAUTHORIZED",
        };

        str.to_string()
    }

    fn get_status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::Gone => StatusCode::GONE,
            Self::GenericError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl Into<WebError> for CommonError {
    fn into(self) -> WebError {
        WebError {
            code: self.get_code(),
            status: self.get_status_code(),
            message: self.to_string(),
        }
    }
}
