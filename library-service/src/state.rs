use std::sync::Mutex;
use sqlx::postgres::PgPool;
use crate::model::book::Book;

pub struct AppState {
    pub probe: String,
    pub library: Mutex<Vec<Book>>,
    pub db: PgPool,
}