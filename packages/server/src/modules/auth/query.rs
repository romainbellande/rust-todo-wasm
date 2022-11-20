use super::{body::AuthBody, credentials::Credentials, service};
use async_graphql::{Context, Object, Result};
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct AuthQuery;

#[Object]
impl AuthQuery {
    async fn login(&self, ctx: &Context<'_>, credentials: Credentials) -> Result<AuthBody> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        service::authorize(conn, credentials).await
    }
}
