use axum::async_trait;
use async_graphql::{Guard, Context, Result, Error, ErrorExtensions};
use crate::utils::errors::WebError;
use super::{errors::AuthError, AccessTokenClaims};
use crate::modules::auth::extractor::AccessToken;

pub struct AuthGuard;

impl AuthGuard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<(), Error> {
        let access_token = ctx.data_opt::<AccessToken>().ok_or({
            let web_error: WebError = AuthError::MissingAccessToken.into();
            web_error.extend()
        })?;

        AccessTokenClaims::from_string(access_token.clone().0.clone()).map_err(|err| err.extend())?;

        Ok(())
    }
}
