use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq)]
pub enum Error {
    /// 401
    #[error("unauthorized")]
    Unauthorized,

    /// 403
    #[error("forbidden")]
    Forbidden,

    /// 404
    #[error("not found")]
    NotFound,

    /// 422
    #[error("unprocessable entity")]
    UnprocessableEntity,

    /// 500
    #[error("internal server error")]
    InternalServerError,

    /// serde deserialize error
    #[error("deserialize error")]
    DeserializeError,

    /// request error
    #[error("http request error")]
    RequestError,

    #[error("no error extension")]
    NoErrorExtension,

    #[error("no error code provided")]
    NoErrorCodeProvided,

    #[error("unknown error code")]
    UnknownErrorCode,

    #[error("provided error code is not a string")]
    ErrorCodeIsNotStr,
}
