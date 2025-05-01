use axum::debug_handler;
use axum::extract::rejection::FailedToBufferBody;
use axum::http::HeaderMap;
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
use crate::auth::seralizers::{RequestUsers, Users, LoginInfo, LoginReponse, Claims};




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
    State(state): State<AppState>,
    Json(login_info): Json<LoginInfo>
) -> impl IntoResponse {
    let username = &login_info.username;
    let password = &login_info.password;

    match is_valid_user(&state, &username, &password).await {
        UserValidationResult::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR.into_response(), 
        UserValidationResult::InvalidCredentials => (StatusCode::UNAUTHORIZED, AnswerJson(json!({
            "status": "error",
            "message": "Invalid credentials"
        }))).into_response(),
        UserValidationResult::Valid => {

            let claims = Claims {
                sub: username.clone(),
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
            };

            let secret_jwt = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

            let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_jwt)) {
                Ok(token) => token,
                Err(e) => {
                    eprint!("Error Generation Token {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            };
            (StatusCode::OK, Json(LoginResponse { token })).into_response()
        } 
    } 
}



#[derive(Debug)]
pub enum UserValidationResult {
    Valid,               // Пользователь существует и пароль верный
    InvalidCredentials,  // Пользователь не существует или пароль неверный
    DatabaseError,       // Ошибка базы данных
}



pub async fn is_valid_user(state: &AppState, username: &str, password: &str) -> UserValidationResult {
    let query = r#"
        SELECT password_hash 
        FROM users 
        WHERE username = $1
    "#;

    let result = match sqlx::query_scalar::<_, String>(query)
        .bind(username)
        .fetch_optional(&state.db_pool)
        .await {
            Ok(result) => result,
            Err(_) => return UserValidationResult::DatabaseError,
        };


    match result {
            Some(hash) => {
                match verify(password, &hash) {
                    Ok(true) => UserValidationResult::Valid,
                    _ => UserValidationResult::InvalidCredentials,
                }
            },
            None => UserValidationResult::InvalidCredentials,
        }
}

async fn get_info_handler(header_map: HeaderMap) -> impl IntoResponse {
    let secret_jwt = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    if let Some(auth_header) = header_map.get("Authorization") {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer ") {
                let token = auth_header_str.trim_start_matches("Bearer ").to_string();

                match decode::<Claims>(&token, &DecodingKey::from_secret(&secret_jwt),  &Validation::default()) {
                    Ok(_) => {
                        let info = "You are valid here is info".to_string();
                        return AnswerJson(info).into_response();
                    }
                    Err(e) => {
                        eprint!("Error Generation Token {}", e);
                        return StatusCode::UNAUTHORIZED.into_response()
                    }
                }
            }
        };
    }
    StatusCode::UNAUTHORIZED.into_response()
}


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