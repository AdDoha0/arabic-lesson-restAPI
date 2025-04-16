use axum::{
    extract::{Json, Path, Query, State},
    http::{response, HeaderMap, HeaderValue, StatusCode},
};
use axum::response::{IntoResponse, Json as AnswerJson};


use crate::lessons::serializers::{Textbook, RequestTextbook, Lesson};
use crate::lessons::state::AppState;
use crate::utils::pagination::{Pagination, PaginateQuery};



impl PaginateQuery for Textbook {}







pub async fn get_all_textbooks(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let query = "SELECT * FROM textbook";

    match Textbook::paginate_query(&state.db_pool, query, &pagination).await {
        Ok(records) => {

            let total_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM textbook")
                .fetch_one(&state.db_pool)
                .await
                .unwrap_or(0);

            let mut response  = AnswerJson(records).into_response();

            // Нужные заголовки для пагинации
            response.headers_mut()
                .append("X-Total-Count", total_count.to_string().parse().unwrap());
            response.headers_mut()
                .append("X-Page", pagination.page.unwrap_or(1).to_string().parse().unwrap());
            response.headers_mut()
                .append("X-Per-Page", pagination.limit.unwrap_or(10).to_string().parse().unwrap());

            response


        },
        Err(err) => {
            eprint!("Failed to get textbooks: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}







pub async fn create_textbook(
    State(state): State<AppState>,
    Json(payload): Json<RequestTextbook>
) -> impl IntoResponse {

    let query = r#"
        INSERT INTO lesson (title, )
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
        },
    }

}



pub async fn update_textbook(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<RequestTextbook>
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
            }
            Err(err) => {
                eprint!("Failed to update textbook: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            },
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



pub async fn get_all_lessons_for_textbook(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = r#"
        SELECT * FROM lesson
        WHERE textbook_id = $1
    "#;

    let result = sqlx::query_as::<_, Lesson>(query)
        .bind(id)
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(lessons) => {
            if lessons.is_empty() {
                StatusCode::NOT_FOUND.into_response()
            } else {
                (StatusCode::OK, AnswerJson(lessons)).into_response()
            }
        },
        Err(err) => {
            eprint!("Failed to get lessons for textbook: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}


