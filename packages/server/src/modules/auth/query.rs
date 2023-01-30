use super::{body::AuthBody, credentials::Credentials, service};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;
use cookie::{Cookie, SameSite, time::Duration};
use crate::CONFIG;

fn create_access_token_cookie<'a>(access_token: &'a String) -> Cookie {
    Cookie::build("access_token", access_token.clone())
        .same_site(SameSite::Strict)
        .secure(CONFIG.is_production())
        .http_only(CONFIG.is_production())
        .path("/")
        .max_age(Duration::seconds(CONFIG.access_token_duration.into()))
        .finish()
}

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn login(&self, ctx: &Context<'_>, credentials: Credentials) -> Result<AuthBody> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let response = service::authorize(conn, credentials).await;

        if let Ok(response) = response.clone() {

            let access_token_value = response.access_token;
            let access_token_cookie = create_access_token_cookie(&access_token_value);
            ctx.insert_http_header("Set-Cookie", access_token_cookie.to_string());
        }

        response
    }
}
