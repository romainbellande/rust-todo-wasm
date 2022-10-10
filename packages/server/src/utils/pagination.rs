use async_graphql::{OutputType, SimpleObject};
use sea_orm::{DatabaseConnection, FromQueryResult, Paginator, SelectModel};
use serde::Serialize;
use std::marker::{Send, Sync};

#[derive(Serialize, SimpleObject, Debug)]
pub struct PaginatedResult<T: Send + Sync + OutputType> {
    pub data: Vec<T>,
    pub num_items: usize,
    pub num_pages: usize,
    pub page: usize,
    pub page_items: usize,
}

impl<T: FromQueryResult + Send + Sync + OutputType> PaginatedResult<T> {
    pub async fn new<'db>(
        paginator: Paginator<'db, DatabaseConnection, SelectModel<T>>,
        page: usize,
        data: Vec<T>,
    ) -> Self {
        let num_pages = paginator.num_pages().await.ok().unwrap();
        let num_items = paginator.num_items().await.ok().unwrap();

        Self {
            page,
            num_pages,
            num_items,
            page_items: data.len(),
            data,
        }
    }
}
