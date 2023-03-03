use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Book>> for Book {
    fn from(book: web::Json<Book>) -> Self {
        Book {
            id: book.id,
            title: book.title.clone(),
            author: book.author.clone(),
            posted_time: book.posted_time,
        }
    }
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}