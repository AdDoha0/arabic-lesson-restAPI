use sqlx::PgPool;

use axum::http::header::{CONTENT_TYPE, HeaderName};


use tower_http::cors::{CorsLayer, Any};


mod lessons;
mod handlers;
mod utils;

use lessons::state::AppState;
use lessons::routes::create_router;






#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");



    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE])
        .expose_headers([HeaderName::from_static("content-range")]);
    // let middleware_stack = ServiceBuilder::new().layer(cors);



    // Здесь создаём роуты и добавляем CORS как слой
    let app = create_router(AppState{ db_pool: db_pool }).layer(cors);



    let listener = tokio::net::TcpListener::bind("0.0.0.0:2000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}