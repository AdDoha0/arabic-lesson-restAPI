
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use axum::response::{IntoResponse, Json as AnswerJson};
use serde::{Deserialize};



use crate::lessons::serializers::{Word, RequestWord};
use crate::lessons::state::AppState;



pub async fn get_words(
    State(state): State<AppState>,
    // Query(params): Query<LessonQuery>
) -> impl IntoResponse {

    let query = "SElECT * FROM word";

    let result = sqlx::query_as::<_, Word>(query)
        .fetch_all(&state.db_pool)
        .await;

    match result {
        Ok(result) => AnswerJson(result).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}




pub async fn get_word(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let query = "SELECT * FROM word WHERE id = $1";

    let result = sqlx::query_as::<_, Word>(query)
       .bind(id)
       .fetch_optional(&state.db_pool)
       .await;

    match result {
        Ok(result) => match result {
            Some(result) => AnswerJson(result).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        }
        Err(err) => {
            eprint!("Failed to get word: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}



pub async fn create_word(
    State(state): State<AppState>,
    Json(payload): Json<RequestWord>
) -> impl IntoResponse {

    let query = r#"
        INSERT INTO word (term, definition, lesson_id)
        VALUES ($1, $2, $3)
        RETURNING id, term, definition, lesson_id
    "#;

    let result = sqlx::query_as::<_, Word>(query)
        .bind(&payload.term)
        .bind(&payload.definition)
        .bind(&payload.lesson_id)
        .fetch_one(&state.db_pool)
        .await;

    match result {
        Ok(result) => AnswerJson(result).into_response(),
        Err(err) => {
            eprint!("Failed to create word: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        },
    }
}





pub async fn update_word_put(
    Path(lesson_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<RequestWord>,
) -> impl IntoResponse {
    let query = r#"
        UPDATE word
        SET term = $1, definition = $2, lesson_id = $3
        WHERE id = $4
        RETURNING id, term, definition, lesson_id
    "#;

    let result = sqlx::query_as::<_, Word>(query)
        .bind(&payload.term)
        .bind(&payload.definition)
        .bind(&payload.lesson_id)
        .bind(lesson_id)
        .fetch_optional(&state.db_pool)
        .await;

    match result {
        Ok(Some(lesson)) => Ok(AnswerJson(lesson)),
        Ok(None) => Err((StatusCode::NOT_FOUND, format!("Lesson with id {} not found", lesson_id))),
        Err(err) => {
            eprintln!("Failed to update lesson: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update lesson".to_string()))
        }
    }
}




pub async fn delete_word(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {

    let query = r#"
        DELETE FROM word
        WHERE id = $1
        RETURNING id
    "#;

    let result = sqlx::query(query)
       .bind(id)
       .fetch_optional(&state.db_pool)
       .await;

    match result {
        Ok(Some(_)) => StatusCode::OK.into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(err) => {
            eprint!("Failed to delete word: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}