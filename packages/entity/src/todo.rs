use async_graphql::*;
use fake::faker::lorem::en::{Sentence, Word};
use fake::Fake;
use field_names::FieldNames;
use sea_orm::{entity::prelude::*, Set, DeleteMany};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject, FieldNames,
)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub title: String,

    pub description: String,

    #[sea_orm(column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub user_id: Uuid,
}

impl Model {
    pub fn new(title: String, description: String, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            user_id
        }
    }

    pub fn new_fake(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: Word().fake::<String>(),
            description: Sentence(3..5).fake::<String>(),
            user_id,
        }
    }

    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.to_owned()),
            title: Set(self.title.to_owned()),
            description: Set(self.description.to_owned()),
            user_id: Set(self.user_id.to_owned())
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(Column::Title.eq(title))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate)]
pub struct CreateTodo {
    pub title: String,

    pub text: String,
}
