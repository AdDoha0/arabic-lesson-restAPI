use sqlx::{query_as, Error, PgPool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}
