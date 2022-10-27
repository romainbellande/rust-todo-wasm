use async_graphql::InputObject;
use serde::Serialize;

#[derive(Serialize, InputObject, Debug)]
pub struct Filter {
    field: String,
    value: String,
    operator: String,
}
