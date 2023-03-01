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

pub async fn post_add_book(
    new_book: web::Json<Book>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    println!("Received new book");
    let book = db_add_book(&app_state.db, new_book.into()).await;
    HttpResponse::Ok().json(book)
}

pub async fn post_bulk_insert(
    new_books: web::Json<Vec<Book>>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let result = db_bulk_insert(&app_state.db, new_books.into_inner()).await;
    if result.unwrap() == true {
        HttpResponse::Ok().json("Bulk insert done.")
    } else {
        HttpResponse::Ok().json("Bulk insert failure.")
    }
}

pub async fn get_books(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, BookError> {
    db_books(&app_state.db)
        .await
        .map(|books| HttpResponse::Ok().json(books))
}

pub async fn get_book_by_id(
    param: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, BookError> {
    let tuple = param.into_inner();
    let id: i32 = tuple;
    db_read_book_by_id(id, &app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn put_book_by_id(
    param: web::Path<i32>,
    updated_book: web::Json<Book>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, BookError> {
    let tuple = param.into_inner();
    let id: i32 = tuple;
    db_update_book_by_id(id, updated_book.into_inner(), &app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn delete_book_by_id(
    param: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, BookError> {
    let tuple = param.into_inner();
    let id: i32 = tuple;
    db_delete_book_by_id(id, &app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json("kkju"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    /*
    #[actix_rt::test]
    async fn test_add_book() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let new_book = Book {
            id: 99,
            title: "Unit Test title".into(),
            author: "Unit Test author".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2023, 02, 28).expect("none").and_hms_opt(06, 30, 0).unwrap()),
        };

        let json_new_book = web::Json(new_book);
        let http_response = post_add_book(json_new_book, shared_data).await;
        assert_eq!(http_response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_bulk_insert() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let new_book_100 = Book {
            id: 100,
            title: "Unit Test title 100".into(),
            author: "Unit Test author 100".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2023, 02, 28).expect("none").and_hms_opt(06, 30, 0).unwrap()),
        };

        let new_book_200 = Book {
            id: 200,
            title: "Unit Test title 200".into(),
            author: "Unit Test author 200".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2023, 02, 28).expect("none").and_hms_opt(06, 30, 0).unwrap()),
        };

        let new_book_300 = Book {
            id: 300,
            title: "Unit Test title 300".into(),
            author: "Unit Test author 300".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2023, 02, 28).expect("none").and_hms_opt(06, 30, 0).unwrap()),
        };

        let stack: Vec<Book> = Vec::from([new_book_100, new_book_200, new_book_300]);

        let json_stack = web::Json(stack);
        let http_response = post_bulk_insert(json_stack, shared_data).await;
        assert_eq!(http_response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_books() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let http_response = get_books(shared_data).await;
        assert_eq!(http_response.is_ok(), true);
    }

    #[actix_rt::test]
    async fn test_book_by_id() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let param: web::Path<(i32)> = web::Path::from(100);
        let http_response = get_book_by_id(param, shared_data).await;
        assert_eq!(http_response.is_ok(), true);
    }

    #[actix_rt::test]
    async fn test_update_book_by_id() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let updated_book = Book {
            id: 99,
            title: "Unit Test title updated".into(),
            author: "Unit Test author updated".into(),
            posted_time: Some(NaiveDate::from_ymd_opt(2023, 02, 28).expect("none").and_hms_opt(06, 30, 0).unwrap()),
        };
        let param: web::Path<(i32)> = web::Path::from(updated_book.id);
        let json_updated_book = web::Json(updated_book);
        let http_response = put_book_by_id(param, json_updated_book, shared_data).await;
        assert_eq!(http_response.is_ok(), true);
    }
    */

    #[actix_rt::test]
    async fn test_delete_book_by_id() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new()
            .idle_timeout(std::time::Duration::from_secs(10))
            .connect(&database_url)
            .await
            .unwrap();
        let shared_data = web::Data::new(AppState {
            probe: "Probe test ok....".to_string(),
            library: Mutex::new(vec![]),
            db: db_pool,
        });

        let param: web::Path<(i32)> = web::Path::from(99);
        let http_response = delete_book_by_id(param, shared_data).await;
        assert_eq!(http_response.is_ok(), true);
    }
}