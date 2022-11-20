use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn delete_many<Entity: EntityTrait>(conn: &DatabaseConnection, name: &'static str) {
    let result = Entity::delete_many().exec(conn).await;

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
}
