use async_graphql::InputObject;
use serde::Deserialize;

#[derive(Debug, Deserialize, InputObject)]
pub struct Credentials {
    pub email: String,

    pub password: String,
}
