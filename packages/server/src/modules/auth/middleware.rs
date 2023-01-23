use crate::utils::errors::WebError;

use super::{AccessTokenClaims, errors::AuthError};
use axum::{
    extract::TypedHeader,
    http::Request,
    middleware::Next,
    response::Response, headers::Authorization,
    RequestPartsExt
};

pub async fn auth_bearer_middleware<B: Send>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, WebError> {
    let (mut parts, body) = req.into_parts();
     let TypedHeader(Authorization(bearer)) = parts.extract()
        .await
        .map_err(|_| AuthError::MissingAccessToken.into())?;

    let claims = AccessTokenClaims::from_bearer(bearer)?;

            format!("Found claims: {:?}", claims);

    let mut req = Request::from_parts(parts, body);
            req.extensions_mut().insert(claims);

            Ok(next.run(req).await)
}
