use crate::utils::errors::WebError;

use super::{errors::AuthError, AccessTokenClaims};
use axum::{http::Request, middleware::Next, response::Response};
use axum_extra::extract::cookie::CookieJar;

pub async fn auth_bearer_middleware<B: Send>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, WebError> {
    let (parts, body) = req.into_parts();

    let cookie = CookieJar::from_headers(&parts.headers);

    let access_token = cookie
        .get("access_token")
        .map(|cookie| cookie.value().to_owned())
        .ok_or(AuthError::MissingAccessToken.into())?;

    let claims = AccessTokenClaims::from_string(access_token)?;

    format!("Found claims: {:?}", claims);

    let mut req = Request::from_parts(parts, body);
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
