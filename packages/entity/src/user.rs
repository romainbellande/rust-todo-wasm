use argon2;
use async_graphql::*;
use sea_orm::{entity::prelude::*, DeleteMany, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    pub email: String,

    pub password_hash: String,
}

impl Model {
    pub fn new(email: String, password: String, salt: String) -> Self {
        let config = argon2::Config::default();
        let password_hash =
            argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap();

        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
        }
    }

    pub fn verify_password(&self, password: String) -> bool {
        argon2::verify_encoded(&self.password_hash, password.as_bytes()).unwrap_or(false)
    }

    pub fn into_active_model(&self) -> ActiveModel {
        ActiveModel {
            id: Set(self.id.to_owned()),
            email: Set(self.email.to_owned()),
            password_hash: Set(self.password_hash.to_owned()),
        }
    }
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 1))]
    pub email: String,

    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, Validate)]
pub struct UserResponse {
    pub id: Uuid,

    pub email: String,
}

impl From<Model> for UserResponse {
    fn from(user: Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::todo::Entity")]
    Todo,
}

impl Related<super::todo::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Todo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_email(email: &str) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(email))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
