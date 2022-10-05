use crate::modules::todo;
use sea_orm::{DatabaseConnection, DbErr};

pub async fn exec(conn: &DatabaseConnection) -> Result<(), DbErr> {
    todo::fixture::exec(conn).await
}
