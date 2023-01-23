use super::{errors::AuthError, keys::KEYS};
use crate::utils::errors::WebError;
use axum::{
    async_trait,
    TypedHeader,
    extract::{FromRequest, FromRequestParts},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, header::HeaderValue},
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use axum_extra::extract::{PrivateCookieJar, cookie};
use crate::CONFIG;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: usize,
}

impl AccessTokenClaims {
    pub fn from_bearer(bearer: Bearer) -> Result<Self, WebError> {
        let token_data = decode::<AccessTokenClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken.into())?;

        Ok(token_data.claims)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AccessTokenClaims
where
    S: Send + Sync
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::InvalidToken.into())?;

        Self::from_bearer(bearer)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub exp: usize,
    // unique identifier for the refresh token to identify it
    // and blacklist it if user logged out
    pub jti: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for RefreshTokenClaims
where
    S: Send + Sync,
{
    type Rejection = WebError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookie = PrivateCookieJar::from_headers(&parts.headers, CONFIG.cookie_key.clone());

        let refresh_token = cookie.get("refresh_token").ok_or_else(|| AuthError::MissingRefreshToken.into())?;

        let token_data = decode::<RefreshTokenClaims>(refresh_token.value(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken.into())?;

        Ok(token_data.claims)
    }
}

