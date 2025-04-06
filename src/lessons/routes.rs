
// Textbook	/textbooks	/textbooks/{id}/lessons
// Lesson	/lessons	/lessons/{id}/words
// Word	/words	или через /lessons/{id}/words для вложений

use axum::{routing::get, Router};

use super::state::AppState;
use crate::handlers::{
    textbook::*
};


async fn root() -> &'static str {
    "Arabic API"
}


pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/textbooks", get(get_all_textbooks).post(create_textbook))
        .route("/textbooks/{id}", get(get_textbook).put(update_textbook).delete(delete_textbook))
        .route("/textbooks/{id}/lessons", get(get_all_lessons_for_textbook))
        .with_state(state)
}