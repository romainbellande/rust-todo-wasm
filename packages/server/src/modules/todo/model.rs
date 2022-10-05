use async_graphql::{Context, Object, ID};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    id: Uuid,
    title: String,
    description: String,
}

#[Object]
impl Todo {
    async fn id(&self) -> ID {
        self.id.into()
    }
}

pub async fn create_todo(_ctx: &Context<'_>, _title: String, _description: String) {}
