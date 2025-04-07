use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
};
use axum::response::{IntoResponse, Json as AnswerJson};


use crate::lessons::serializers::{Textbook, RequestTextbook, Lesson};
use crate::lessons::state::AppState;


pub async fn get_all_textbooks(
    State(state): State<AppState>,
) -> impl IntoResponse {

    let query = "SElECT * FROM textbook";

    let result = sqlx::query_as::<_, Textbook>(query)
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(result) => AnswerJson(result).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}



pub async fn get_textbook(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {

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
        Ok(updated_textbook) => (StatusCode::OK, AnswerJson(updated_textbook)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprint!("Failed to update textbook: {:?}", err );
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


