use async_trait::async_trait;
use sqlx::{postgres::PgRow, FromRow, PgPool, Postgres, QueryBuilder};

use axum::response::Response;
use axum::http::HeaderValue;


pub trait HasPagination {
    fn page(&self) -> Option<i64>;
    fn limit(&self) -> Option<i64>;
}


pub enum PaginateResult<T> {
    Success(Vec<T>),
    NotFound,
}


#[async_trait]
pub trait PaginateQuery
where
    Self: Sized + for<'r> FromRow<'r, PgRow> + Send + Unpin + 'static,
{
    /// Универсальный метод пагинации
    async fn paginate_query<'a, T: HasPagination + Send + Sync>(
        db_pool: &PgPool,
        mut builder: QueryBuilder<'a, Postgres>,
        params: &T,
    ) -> Result<PaginateResult<Self>, sqlx::Error> {
        // Получаем SQL-запрос, чтобы подсчитать количество записей
        let count_sql = format!(
            "SELECT COUNT(*) FROM ({}) AS subquery",
            builder.sql()
        );

        // Считаем общее количество
        let total_count: i64 = sqlx::query_scalar(&count_sql)
            .fetch_one(db_pool)
            .await
            .unwrap_or(0);

        // Если заданы параметры пагинации
        if let (Some(page), Some(limit)) = (params.page(), params.limit()) {
            let offset = (page - 1) * limit;

            if offset >= total_count {
                return Ok(PaginateResult::NotFound);
            }

            builder.push(" LIMIT ").push_bind(limit);
            builder.push(" OFFSET ").push_bind(offset);
        }

        let query = builder.build_query_as::<Self>();
        let records = query.fetch_all(db_pool).await?;
        Ok(PaginateResult::Success(records))
    }


    fn add_pagination_headers<T: HasPagination>(
        mut response: Response,
        total_count: i64,
        params: &T,
    ) -> Response {
        if let Ok(count_header) = HeaderValue::from_str(&total_count.to_string()) {
            response.headers_mut().insert("X-Total-Count", count_header);
        }

        if let Ok(page_header) = HeaderValue::from_str(&params.page().unwrap_or(1).to_string()) {
            response.headers_mut().insert("X-Page", page_header);
        }

        if let Ok(limit_header) = HeaderValue::from_str(&params.limit().unwrap_or(10).to_string()) {
            response.headers_mut().insert("X-Per-Page", limit_header);
        }

        response
    }



}





