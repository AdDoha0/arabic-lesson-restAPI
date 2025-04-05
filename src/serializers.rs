use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
}


#[derive(Deserialize)]
pub struct RequestItem {
    pub name: String,
    pub description: String,
}
