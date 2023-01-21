use crate::utils::errors::CommonError;
use crate::utils::{pagination::PaginatedResult, Filter};

use async_graphql::{Context, Object, Result};
use entity::todo;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait};

const DEFAULT_LIMIT: usize = 20;
const DEFAULT_PAGE: usize = 0;

#[derive(Default)]
pub struct TodoQuery;

#[Object]
impl TodoQuery {
    async fn todos(
        &self,
        ctx: &Context<'_>,
        page: Option<usize>,
        limit: Option<usize>,
        _filters: Option<Vec<Filter>>,
    ) -> Result<PaginatedResult<todo::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        println!("{:?}", todo::Model::FIELDS);
        paginate(page, limit, conn).await
    }
}

async fn paginate(
    page: Option<usize>,
    limit: Option<usize>,
    conn: &DatabaseConnection,
) -> Result<PaginatedResult<todo::Model>> {
    let page = page.unwrap_or(DEFAULT_PAGE);
    let limit = limit.unwrap_or(DEFAULT_LIMIT);

    let finder = todo::Entity::find();
    let paginator = finder.paginate(conn, limit);

    let data = paginator
        .fetch_page(page)
        .await
        .map_err(|db_err| CommonError::InternalServerError(db_err.to_string()))?;

    Ok(PaginatedResult::new(paginator, page, data).await)
}
