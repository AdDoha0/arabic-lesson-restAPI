use serde::{Serialize, Deserialize};
use sqlx::{FromRow};
// use chrono::NaiveDateTime;
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, FromRow)]
pub struct Textbook {
    pub id: i32,
    pub title: String,
    pub description: String
}


#[derive(Serialize, FromRow)]
pub struct Lesson {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub video_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub textbook_id: i32,
}



#[derive(Serialize, FromRow)]
pub struct Word {
    pub id: i32,
    pub term: String,
    pub definition: String,
    pub lesson_id: i32,
}

// ------------------------------request-----------------------------------------------------------

#[derive(Deserialize)]
pub struct RequestTextbook {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct RequestLesson {
    pub title: String,
    pub text: String,
    pub video_url: Option<String>,
    pub textbook_id: i32,
}

#[derive(Deserialize)]
pub struct RequestWord {
    pub term: String,
    pub definition: String,
    pub lesson_id: i32,
}



// --------------------------------path method----------------------------------------------------
#[derive(Deserialize)]
pub struct PatchLesson {
    pub title: Option<String>,
    pub text: Option<String>,
    pub video_url: Option<String>,
    pub textbook_id: Option<i32>,
}