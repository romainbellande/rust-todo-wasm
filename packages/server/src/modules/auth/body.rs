use async_graphql::SimpleObject;
use serde::Serialize;

#[derive(Debug, Serialize, SimpleObject)]
pub struct AuthBody {
    pub access_token: String,

    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
