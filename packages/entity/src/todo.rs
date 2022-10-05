use async_graphql::*;
use fake::faker::lorem::en::{Sentence, Word};
use fake::Fake;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub title: String,

    pub description: String,
}

impl Model {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
        }
    }

    pub fn new_fake() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Word().fake::<String>(),
            description: Sentence(3..5).fake::<String>(),
        }
    }

    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.to_owned()),
            title: Set(self.title.to_owned()),
            description: Set(self.description.to_owned()),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate)]
pub struct CreateTodo {
    pub title: String,

    pub text: String,
}
