use std::{clone, net::TcpListener};

use axum::{routing::{get, post}, Router};
use sqlx::{PgPool};

mod state;
mod serializers;
mod handlers;

use state::AppState;
use crate::handlers::*;



async fn root() -> &'static str {
    "Arabic API"
}



#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app = Router::new()
        .route("/", get(root))
        .route("/items", post(create_item))
        .with_state(AppState { db_pool: db_pool });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}