use super::create_error_enum::create_error_enum;

create_error_enum!(
    CommonError,
    [
        InternalServerError,
        BadRequest,
        Forbidden,
        Gone,
        GenericError,
        RequestTimeout,
        Unauthorized
    ]
);

