use serde::{Serialize, Deserialize};
use sqlx::{FromRow};




#[derive(Serialize, FromRow)]
pub struct Textbook {
    pub id: u32,
    pub title: String,
    pub description: String
}


#[derive(Serialize, FromRow)]
pub struct Lesson {
    pub id: u32,
    pub title: String,
    pub text: String,
    pub video_url: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub volume: u32,
}



#[derive(Serialize, FromRow)]
pub struct Word {
    pub id: u32,
    pub term: String,
    pub definition: String,
    pub lesson: u32,
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
    pub volume: u32,
}

#[derive(Deserialize)]
pub struct RequestWord {
    pub term: String,
    pub definition: String,
    pub lesson: u32,
}