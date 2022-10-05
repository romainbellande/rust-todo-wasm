use crate::CONFIG;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new() -> Self {
        Database {
            database_url: CONFIG.database_url.clone(),
        }
    }

    pub async fn get_connection(&self) -> DatabaseConnection {
        sea_orm::Database::connect(&self.database_url)
            .await
            .unwrap_or_else(|_| panic!("could not connect to database: {}", self.database_url))
    }
}
