use serde::{Deserialize};

#[derive(Deserialize)]
pub struct LessonQuery {
    pub textbook_id: Option<i32>,
}


#[derive(Deserialize)]
pub struct WordQuery {
    pub lesson_id: Option<i32>,
}