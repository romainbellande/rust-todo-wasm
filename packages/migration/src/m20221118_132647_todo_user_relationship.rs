use sea_orm_migration::prelude::*;
use entity::{user, todo};

const FK_TODO_USER_ID: &'static str = "FK_TODO_USER_ID_0429e73d-e88e-4574-8db4-d0d45015eee0";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let todo_table = Table::alter()
            .table(todo::Entity)
            .add_column(ColumnDef::new(todo::Column::UserId).uuid().not_null())
            .to_owned();

        let user_id_fk = ForeignKey::create()
            .name(FK_TODO_USER_ID)
            .from(todo::Entity, todo::Column::UserId)
            .to(user::Entity, user::Column::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager.alter_table(todo_table).await?;
        manager.create_foreign_key(user_id_fk).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let todo_table = Table::alter()
            .table(todo::Entity)
            .drop_column(todo::Column::UserId)
            .to_owned();

        let drop_user_id_fk = ForeignKey::drop()
            .name(FK_TODO_USER_ID)
            .table(todo::Entity)
            .to_owned();

        manager.alter_table(todo_table).await?;
        manager.drop_foreign_key(drop_user_id_fk).await
    }
}

