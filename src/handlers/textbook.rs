use axum::debug_handler;
use axum::response::{IntoResponse, Json as AnswerJson};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};

use serde::Deserialize;
use sqlx::QueryBuilder;

use crate::lessons::serializers::{RequestTextbook, Textbook};
use crate::lessons::state::AppState;
use crate::utils::pagination::{HasPagination, PaginateQuery, PaginateResult};

impl PaginateQuery for Textbook {}

#[derive(Deserialize)]
pub struct TextbookQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>
}

impl HasPagination for TextbookQuery {
    fn page(&self) -> Option<i64> {
        self.page
    }

    fn limit(&self) -> Option<i64> {
        self.limit
    }
}

#[debug_handler]
pub async fn get_all_textbooks(
    State(state): State<AppState>,
    Query(params): Query<TextbookQuery>,
) -> impl IntoResponse {
    let mut builder = QueryBuilder::new("SELECT * FROM textbook WHERE 1=1");

    builder.push(" ORDER BY id");

    match Textbook::paginate_query(&state.db_pool, builder, &params).await {
        Ok(PaginateResult::Success(records)) => {
            let mut response = AnswerJson(records).into_response();

            let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM textbook")
                .fetch_one(&state.db_pool)
                .await
                .unwrap_or(0);

            response = Textbook::add_pagination_headers(response, total_count, &params);

            response
        }
        Ok(PaginateResult::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("Failed to get textbooks: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}



pub async fn get_textbook(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let query = r#"
        SELECT * FROM textbook
        WHERE id = $1
    "#;

    let result = sqlx::query_as::<_, Textbook>(query)
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(textbook)) => (StatusCode::OK, AnswerJson(textbook)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_textbook(
    State(state): State<AppState>,
    Json(payload): Json<RequestTextbook>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO textbook (title, description)
        VALUES ($1, $2)
        RETURNING id, title, description
    "#;

    let result = sqlx::query_as::<_, Textbook>(query)
        .bind(&payload.title)
        .bind(&payload.description)
        .fetch_one(&state.db_pool)
        .await;

    match result {
        Ok(textbook) => (StatusCode::CREATED, AnswerJson(textbook)).into_response(),
        Err(err) => {
            eprint!("Failed to create textbook: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_textbook(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<RequestTextbook>,
) -> impl IntoResponse {
    let query = r#"
        UPDATE textbook
        SET title = $1, description = $2
        WHERE id = $3
        RETURNING id, title, description
    "#;

    let result = sqlx::query_as::<_, Textbook>(query)
        .bind(&payload.title)
        .bind(&payload.description)
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(result) => match result {
            Some(result) => AnswerJson(result).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        Err(err) => {
            eprint!("Failed to update textbook: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn delete_textbook(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = r#"
        DELETE FROM textbook
        WHERE id = $1
        RETURNING id
    "#;

    let result = sqlx::query(query)
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(_)) => StatusCode::NO_CONTENT.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprint!("Failed to delete textbook {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
