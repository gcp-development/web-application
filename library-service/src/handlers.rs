use super::dao::*;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use super::model::Book;
use chrono::Utc;
use crate::errors::BookError;

pub async fn get_probe(app_state: web::Data<AppState>) -> HttpResponse {
    let probe_response = &app_state.probe;
    HttpResponse::Ok().json(&probe_response)
}

pub async fn new_book(
    new_book: web::Json<Book>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new book");
    let book = add_book(&app_state.db, new_book.into()).await;
    HttpResponse::Ok().json(book)
}

pub async fn bulk(
    new_books: web::Json<Vec<Book>>,
    app_state: web::Data<AppState>,
) -> HttpResponse {

    let result = bulk_insert(&app_state.db,new_books.into_inner()).await;
    HttpResponse::Ok().json("Books added.")
}

pub async fn update_book_by_id(
    updated_book: web::Json<Book>,
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let id = params.into_inner();
    let selected_book = app_state
        .library
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.id == id)
        .ok_or("Book not found");

    if selected_book.is_ok() == true {
        let reset_book = Book {
            id,
            title: updated_book.title.clone(),
            author: updated_book.author.clone(),
            posted_time: Some(Utc::now().naive_utc()),
        };
        let index = app_state.library.lock().unwrap().iter().position(|x| x.id == selected_book.as_ref().unwrap().id).unwrap();
        app_state.library.lock().unwrap()[index] = reset_book;
        HttpResponse::Ok().json("Book updated".to_string())
    } else {
        HttpResponse::Ok().json("Book not found".to_string())
    }
}

pub async fn get_book_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let id = params.into_inner();
    let selected_book = app_state
        .library
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.id == id)
        .ok_or("Book not found");

    if let Ok(book) = selected_book {
        HttpResponse::Ok().json(book)
    } else {
        HttpResponse::Ok().json("Book not found".to_string())
    }
}

pub async fn delete_book_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    let id = params.into_inner();

    let selected_book = app_state
        .library
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.id == id)
        .ok_or("Book not found");

    if selected_book.is_ok() == true {
        let index = app_state.library.lock().unwrap().iter().position(|x| x.id == selected_book.as_ref().unwrap().id).unwrap();
        app_state.library.lock().unwrap().remove(index);
        HttpResponse::Ok().json("Book not deleted".to_string())
    } else {
        HttpResponse::Ok().json("Book not found".to_string())
    }
}

pub async fn get_all_books(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, BookError> {
    get_books(&app_state.db)
        .await
        .map(|books| HttpResponse::Ok().json(books))
}