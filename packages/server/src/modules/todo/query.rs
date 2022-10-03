use super::model::Todo;
use async_graphql::Object;

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todos(&self) -> Vec<Todo> {
        todo!();
        vec![]
    }
}
