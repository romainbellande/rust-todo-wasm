use async_graphql::{Context, Object, Result};
use entity::todo;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<todo::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = todo::Entity::find()
            .all(conn)
            .await
            .map_err(|db_err| db_err.to_string());
        Ok(result?)
    }
}
