use axum::debug_handler;
use axum::extract::rejection::FailedToBufferBody;
use axum::response::{IntoResponse, Json as AnswerJson};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::ser::Impossible;
use serde_json::json;
use std::env;
use bcrypt::{hash, verify};


use crate::lessons::state::AppState;
use crate::auth::seralizers::{RequestUsers, Users, LoginInfo, LoginReponse};




pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RequestUsers>
) -> impl IntoResponse {


    let check_result_password = validate_password(&payload.password);
    if check_result_password.0 == false {
        return (StatusCode::BAD_REQUEST, AnswerJson(json!({"error": check_result_password.1}))).into_response();
    }

    // Проверяем имя пользователя
    let check_result_username = validate_username(&payload.username);
    if check_result_username.0 == false {
        return (StatusCode::BAD_REQUEST, AnswerJson(json!({"error": check_result_username.1}))).into_response();
    }


    
    
    let query =   r#"
    INSERT INTO users (username, password_hash, email, created_at, updated_at)
    VALUES ($1, $2, $3, NOW(), NOW())
    RETURNING id, username, password_hash, email, created_at, updated_at
    "#;

    let hashed = match hash(&payload.password, 4) {
        Ok(h) => h,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Ошибка хеширования пароля").into_response(),
    };

    let result = sqlx::query_as::<_, Users>(query)
        .bind(&payload.username)
        .bind(hashed)
        .bind(&payload.email)
        .fetch_one(&state.db_pool)
        .await;
    

    match result {
        Ok(u) => (StatusCode::OK, AnswerJson(u)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response()   
    }
    
}


async fn login(
    Json(login_info): Json<LoginInfo>
) -> impl IntoResponse {
    let username = &login_info.username;
    let password = &login_info.password;



}



pub async fn is_valid_user(state: &AppState, username: &str, password: &str) -> Result<bool, sqlx::Error> {
    let query = r#"
        SELECT password_hash 
        FROM users 
        WHERE username = $1
    "#;

    let result = sqlx::query_scalar::<_, String>(query)
        .bind(username)
        .fetch_optional(&state.db_pool)
        .await?;

    match result {
        Some(hash) => Ok(verify(password, &hash).unwrap_or(false)),
        None => Ok(false)
    }
}


// async fn get_info_handler() -> impl IntoResponse {

// }





pub fn validate_username(username: &str) -> (bool, String) {
    // Простая проверка на пустые значения
    if username.is_empty() {
        return (false, "the username field must not be empty".to_string());
    }

    // Проверка длины имени пользователя (минимум 3 символа)
    if username.len() < 3 {
        return (false, "Username length cannot be less than three characters".to_string());
    }

    // Проверка, что имя пользователя содержит только допустимые символы
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return (false, "username contains unacceptable characters".to_string());
    }

    (true, "Everything is correct".to_string())
}


pub fn validate_password(password: &str) -> (bool, String) {

    if password.is_empty() {
        return (false, "the password field must not be empty".to_string());
    }

    // Проверка длины пароля (минимум 6 символов)
     if password.len() < 6 {
        return (false, "the password field must be more than 6 characters".to_string());
    }
    // Проверка, что пароль содержит хотя бы одну цифру и одну букву
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_letter = password.chars().any(|c| c.is_alphabetic());

    if has_digit && has_letter == false {
        return (false, "The password must contain at least one number and one letter".to_string());
    }
    (true, "Everything is correct".to_string())
}