use crate::utils::times;
use entity::todo;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

pub async fn exec(conn: &DatabaseConnection) -> Result<(), DbErr> {
    let name: String = "todo".to_string();
    let result = todo::Entity::delete_many().exec(conn).await;

    match result.clone() {
        Ok(delete_resut) => {
            println!(
                "[{}] table cleaned, {} rows affected",
                name, delete_resut.rows_affected
            );
        }
        Err(err) => {
            println!("[{name}] {err}");
        }
    };

    let models = times(50, |_| todo::Model::new_fake().into_active_model());

    todo::Entity::insert_many(models.clone()).exec(conn).await?;

    println!("[{name}] fixture loaded with {} items", models.len());

    Ok(())
}
