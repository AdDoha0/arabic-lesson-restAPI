use serde::Deserialize;

use async_trait::async_trait;
use sqlx::{postgres::PgRow, FromRow, PgPool};



#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}


// "Я хочу, чтобы любой тип, который использует PaginateQuery, умел строиться из строки SQL,
// можно было использовать его в асинхронном коде, и при этом он не использовал временные ссылки".

#[async_trait]
pub trait PaginateQuery
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Send + Unpin + 'static,
{
    async fn paginate_query(
        db_pool: &PgPool,
        base_query: &str,
        pagination: &Pagination,
    ) -> Result<Vec<Self>, sqlx::Error> {
        if pagination.page.is_none() || pagination.limit.is_none() {
            return sqlx::query_as::<_, Self>(base_query)
                .fetch_all(db_pool)
                .await;
        }

        let page = pagination.page.unwrap_or(1);
        let limit = pagination.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        let paginated_query = format!("{} LIMIT $1 OFFSET $2", base_query);

        sqlx::query_as::<_, Self>(&paginated_query)
            .bind(limit)
            .bind(offset)
            .fetch_all(db_pool)
            .await
    }
}