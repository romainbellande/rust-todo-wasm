use entity::user;
use sea_orm::{DatabaseConnection, EntityTrait, DbErr, prelude::Uuid};
use crate::utils::fixture;
use crate::CONFIG;

pub async fn exec(conn: &DatabaseConnection) -> Result<Uuid, DbErr> {
    let name = "user";
    fixture::delete_many::<user::Entity>(conn, name).await;

    let model = user::Model::new(CONFIG.admin_email.clone(), CONFIG.admin_password.clone(), CONFIG.salt.clone()).into_active_model();
 
    let user_id = user::Entity::insert(model.clone()).exec(conn).await?.last_insert_id;

    println!("[{name}] fixture loaded with {} items", 1);

    Ok(user_id)
} 
