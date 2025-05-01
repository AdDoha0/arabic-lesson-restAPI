use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use axum::http::{HeaderName, HeaderValue};
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

mod handlers;
mod lessons;
mod utils;
mod auth;

use lessons::routes::create_router;
use lessons::state::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([CONTENT_TYPE, ACCEPT, AUTHORIZATION])
        .allow_credentials(true)
        .expose_headers([HeaderName::from_static("content-range")]);

    let app = create_router(AppState { db_pool: db_pool }).layer(cors);

    println!("Server running on http://0.0.0.0:2000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
