use std::sync::Mutex;
use sqlx::postgres::PgPool;

pub struct AppState {
    pub probe: String,
    pub db: PgPool,
}