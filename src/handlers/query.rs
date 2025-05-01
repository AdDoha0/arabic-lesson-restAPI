use serde::Deserialize;

#[derive(Deserialize)]
pub struct WordQuery {
    pub lesson_id: Option<i32>,
}
