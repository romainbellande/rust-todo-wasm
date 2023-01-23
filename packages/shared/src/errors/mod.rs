mod create_error_enum;
mod user;
use create_error_enum::create_error_enum;

create_error_enum!(
    AppError,
    [
        UnprocessableEntity,
        DeserializeError,
        RequestError,
        // erro extension
        NoErrorExtension,
        NoErrorCodeProvided,
        UnknownErrorCode,
        ErrorCodeIsNotStr,
        // http
        InternalServerError,
        BadRequest,
        Forbidden,
        Gone,
        GenericError,
        RequestTimeout,
        Unauthorized,
        NotFound,
        // user
        UserNotFound,
        CouldNotSaveUser, // auth
        WrongCredentials,
        MissingCredentials,
        InvalidToken,
        TokenCreation,
        MissingRefreshToken,
        MissingAccessToken
    ]
);
