
// Textbook	/textbooks	/textbooks/{id}/lessons
// Lesson	/lessons	/lessons/{id}/words
// Word	/words	или через /lessons/{id}/words для вложений

use axum::{routing::get, Router};

use super::state::AppState;
use crate::handlers::{
    textbook::*,
    lesson::*
};


async fn root() -> &'static str {
    "Arabic API"
}


pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/", get(root))
        //-------------------------------textbooks-------------------------------------------------
        .route("/api/v1/textbooks", get(get_all_textbooks).post(create_textbook))
        .route("/api/v1/textbooks/{id}", get(get_textbook).put(update_textbook).delete(delete_textbook))
        .route("/api/v1/textbooks/{id}/lessons", get(get_all_lessons_for_textbook))
        //-------------------------------lessons---------------------------------------------------
        .route("/api/v1/lessons", get(get_lessons).post(create_lesson))
        .route("/api/v1/lessons/{id}", get(get_leson).patch(update_lesson_patch).delete(delete_lesson))


        .with_state(state)
}