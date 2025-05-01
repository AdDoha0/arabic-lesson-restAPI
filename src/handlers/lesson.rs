use std::thread::Builder;

use axum::response::{IntoResponse, Json as AnswerJson};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::QueryBuilder;

use crate::lessons::serializers::{Lesson, NewWord, PatchLesson, RequestLesson, Word};
use crate::lessons::state::AppState;
use crate::utils::pagination::{HasPagination, PaginateQuery, PaginateResult};

impl PaginateQuery for Lesson {}

#[derive(Deserialize)]
pub struct LessonQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub textbook_id: Option<i32>,
}

impl HasPagination for LessonQuery {
    fn page(&self) -> Option<i64> {
        self.page
    }

    fn limit(&self) -> Option<i64> {
        self.limit
    }
}

#[axum::debug_handler]
pub async fn get_lessons(
    State(state): State<AppState>,
    Query(params): Query<LessonQuery>,
) -> impl IntoResponse {
    let mut builder = QueryBuilder::new("SELECT * FROM lesson WHERE 1=1");

    if let Some(id) = params.textbook_id {
        builder.push(" AND textbook_id = ").push_bind(id);
    }

    builder.push(" ORDER BY id");

    let lesson_result = Lesson::paginate_query(&state.db_pool, builder, &params).await;

    match lesson_result {
        Ok(PaginateResult::Success(recods)) => {
            let mut response = AnswerJson(recods).into_response();

            let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM lesson")
                .fetch_one(&state.db_pool)
                .await
                .unwrap_or(0);

            response = Lesson::add_pagination_headers(response, total_count, &params);

            response
        }
        Ok(PaginateResult::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprintln!("DB error: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_leson(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    let query = r#"
            SELECT * FROM lesson
            WHERE id = $1
        "#;

    let result = sqlx::query_as::<_, Lesson>(query)
        .bind(id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(result)) => (StatusCode::OK, AnswerJson(result)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_lesson(
    State(state): State<AppState>,
    Json(payload): Json<RequestLesson>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO lesson (title, text, video_url, textbook_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, title, text, video_url, created_at, textbook_id
    "#;

    let result = sqlx::query_as::<_, Lesson>(query)
        .bind(&payload.title)
        .bind(&payload.text)
        .bind(&payload.video_url)
        .bind(&payload.textbook_id)
        .fetch_one(&state.db_pool)
        .await;

    match result {
        Ok(result) => (StatusCode::CREATED, AnswerJson(result)).into_response(),
        Err(err) => {
            eprint!("Failed to create lesson: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn update_lesson_patch(
    Path(lesson_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<PatchLesson>,
) -> impl IntoResponse {
    // Проверка наличия полей для обновления
    if payload.title.is_none()
        && payload.text.is_none()
        && payload.video_url.is_none()
        && payload.textbook_id.is_none()
    {
        return Err((StatusCode::BAD_REQUEST, "No fields to update".to_string()));
    }

    let query = r#"
        UPDATE lesson SET
            title = COALESCE($1, title),
            text = COALESCE($2, text),
            video_url = COALESCE($3, video_url),
            textbook_id = COALESCE($4, textbook_id)
        WHERE id = $5
        RETURNING id, title, text, video_url, created_at, textbook_id
        "#;

    let result = sqlx::query_as::<_, Lesson>(query)
        .bind(&payload.title)
        .bind(&payload.text)
        .bind(&payload.video_url)
        .bind(&payload.textbook_id)
        .bind(&lesson_id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(lesson)) => Ok(AnswerJson(lesson)),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            format!("Lesson with id {} not found", lesson_id),
        )),
        Err(err) => {
            eprintln!("Failed to update lesson: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to update lesson".to_string(),
            ))
        }
    }
}

pub async fn delete_lesson(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = r#"
        DELETE FROM lesson
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
            eprint!("Failed to delete lesson {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn get_all_word_for_lesson(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = r#"
        SELECT * FROM word
        WHERE lesson_id = $1
    "#;

    let result = sqlx::query_as::<_, Word>(query)
        .bind(id)
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(words) => {
            if words.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                (StatusCode::OK, AnswerJson(words)).into_response()
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch words: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

pub async fn add_word_to_lesson(
    State(state): State<AppState>,
    Path(lesson_id): Path<i32>,
    Json(payload): Json<NewWord>,
) -> impl IntoResponse {
    let query = r#"
        INSERT INTO word (term, definition, lesson_id)
        VALUES ($1, $2, $3)
        RETURNING id, term, definition, lesson_id
    "#;

    let result = sqlx::query_as::<_, Word>(query)
        .bind(&payload.term)
        .bind(&payload.definition)
        .bind(lesson_id)
        .fetch_one(&state.db_pool)
        .await;

    match result {
        Ok(word) => (StatusCode::CREATED, AnswerJson(word)).into_response(),
        Err(err) => {
            eprintln!("Failed to insert word: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
