use super::model::Todo;
use async_graphql::Object;

#[derive(Default)]
pub struct TodoMutation;

#[Object]
impl TodoMutation {
    async fn create_todo(&self) -> Todo {
        todo!()
    }
}
