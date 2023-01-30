use super::{errors::AuthError, keys::KEYS};
use crate::utils::errors::WebError;
use axum::{
    headers::authorization::Bearer,
};
use jsonwebtoken::{decode, Validation};
use serde::{Deserialize, Serialize};
use axum_extra::extract::{PrivateCookieJar, cookie};
use crate::CONFIG;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub sub: String,
    pub exp: i64,
}

impl AccessTokenClaims {
    pub fn from_bearer(bearer: Bearer) -> Result<Self, WebError> {
        Self::from_string(bearer.token().to_string())
    }

    pub fn from_string(value: String) -> Result<Self, WebError> {
        let token_data = decode::<Self>(&value, &KEYS.decoding, &Validation::default())
            .map_err(|err| {
                println!("{:?}", err);
                AuthError::InvalidToken.into()
            })?;

        Ok(token_data.claims)
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub sub: String,
    pub exp: i64,
    // unique identifier for the refresh token to identify it
    // and blacklist it if user logged out
    pub jti: String,
}

impl RefreshTokenClaims {
    pub fn from_string(value: String) -> Result<Self, WebError> {
        let token_data = decode::<Self>(&value, &KEYS.decoding, &Validation::default())
            .map_err(|err| {
                println!("{:?}", err);
                AuthError::InvalidToken.into()
            })?;

        Ok(token_data.claims)
    }
}
