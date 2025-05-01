use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::chrono::NaiveDateTime;



#[derive(Serialize, FromRow)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime 
}

#[derive(Deserialize, FromRow)]
pub struct RequestUsers {
    pub username: String,
    pub password: String,
    pub email: String,
}


#[derive(serde::Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginReponse {
    pub token: String
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
