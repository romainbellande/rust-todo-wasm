use crate::utils::fixture;
use crate::utils::times;
use entity::todo;
use sea_orm::{entity::prelude::Uuid, DatabaseConnection, DbErr, EntityTrait};

pub async fn exec(conn: &DatabaseConnection, fake_user_id: Uuid) -> Result<(), DbErr> {
    let name = "todo";
    fixture::delete_many::<todo::Entity>(conn, name).await;

    let models = times(50, |_| {
        todo::Model::new_fake(fake_user_id).into_active_model()
    });

    todo::Entity::insert_many(models.clone()).exec(conn).await?;

    println!("[{name}] fixture loaded with {} items", models.len());

    Ok(())
}
