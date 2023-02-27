use super::model::Book;
use std::sync::Mutex;
use sqlx::postgres::PgPool;

pub struct AppState {
    pub probe: String,
    pub library: Mutex<Vec<Book>>,
    pub db: PgPool,
}