use crate::modules::{user, todo};
use sea_orm::{DatabaseConnection, DbErr};

pub async fn exec(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let admin_id = user::fixture::exec(conn).await?;
    todo::fixture::exec(conn, admin_id).await
}
