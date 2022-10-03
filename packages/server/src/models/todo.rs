use async_graphql::{Context, ID};

#[derive(Clone)]
pub struct Todo {
    id: ID,
    title: String,
    description: String,
}

pub async fn create_todo(_ctx: &Context<'_>, _title: String, _description: String) {}
