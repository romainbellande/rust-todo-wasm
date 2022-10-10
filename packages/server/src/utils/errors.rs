

use async_graphql::{Error, ErrorExtensions};
use axum::http::StatusCode;

#[derive(Debug, Clone, thiserror::Error)]
pub enum WebError {
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

impl ErrorExtensions for WebError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            WebError::NotFound => {
                e.set("code", StatusCode::NOT_FOUND.as_u16());
                e.set(
                    "message",
                    StatusCode::NOT_FOUND
                        .canonical_reason()
                        .unwrap_or("Unkown"),
                )
            }
            WebError::ServerError(reason) => {
                e.set("code", StatusCode::INTERNAL_SERVER_ERROR.as_u16());
                e.set("message", reason.clone())
            }
            WebError::ErrorWithoutExtensions => {}
        })
    }
}
