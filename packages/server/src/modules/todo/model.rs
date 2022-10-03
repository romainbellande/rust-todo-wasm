use async_graphql::{Context, Object, ID};

#[derive(Clone)]
pub struct Todo {
    id: ID,
    title: String,
    description: String,
}

#[Object]
impl Todo {
    async fn id(&self) -> ID {
        self.id.clone()
    }
}

pub async fn create_todo(_ctx: &Context<'_>, _title: String, _description: String) {}
