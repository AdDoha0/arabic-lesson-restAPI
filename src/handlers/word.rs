
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

}




pub async fn get_word(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {}



pub async fn create_word(
    State(state): State<AppState>,
    Json(payload): Json<RequestWord>
) -> impl IntoResponse {}





pub async fn update_lesson_put(
    Path(lesson_id): Path<i32>,
    State(state): State<AppState>,
    Json(payload): Json<RequestWord>,
) -> impl IntoResponse {}




pub async fn delete_word(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {}