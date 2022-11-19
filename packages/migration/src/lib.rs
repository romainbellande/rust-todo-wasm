pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20221115_200038_user;
mod m20221118_132647_todo_user_relationship;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20221115_200038_user::Migration),
            Box::new(m20221118_132647_todo_user_relationship::Migration),
        ]
    }
}
