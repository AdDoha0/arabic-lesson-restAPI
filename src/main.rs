use std::{clone, net::TcpListener};

use axum::Router;
use serde::Serialize;
use sqlx::{FromRow, PgPool};



#[derive(Serialize, FromRow)]
struct Item {
    id: i32,
    name: String,
    description: String,
}


#[derive(Serialize)]
struct RequestItem {
    name: String,
    description: String,
}


#[derive(Clone)]
struct AppState {
    dp_pool: PgPool,
}

impl AppState {

}



#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");


    let app_state = AppState {
        dp_pool: db_pool,
    };

    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


}