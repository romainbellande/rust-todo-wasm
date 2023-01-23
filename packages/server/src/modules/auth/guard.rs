use axum::async_trait;
use async_graphql::{Guard, Context, Result, Error, ErrorExtensions};
use crate::graphql;
use crate::utils::errors::WebError;
use super::errors::AuthError;

pub struct AuthGuard;

impl AuthGuard {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Guard for AuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
      let access_token = ctx.data_opt::<graphql::AccessToken>();

        if access_token.is_some() {
            Ok(())
        } else {
            let web_error: WebError = AuthError::MissingAccessToken.into();
            Err(web_error.extend())
        }
    }
}
