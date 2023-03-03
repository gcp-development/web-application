use actix_web::{web, HttpResponse};
use crate::dal::book::*;
use crate::errors::BookError;
use crate::model::book::Book;
use crate::state::AppState;

pub async fn post_add_book(
    new_book: web::Json<Book>,
    app_state: web::Data<AppState>,
) ->  Result<HttpResponse, BookError> {
    db_add_book(&app_state.db, new_book.into()).await
}

pub async fn post_bulk_insert(
    new_books: web::Json<Vec<Book>>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, BookError> {
    db_bulk_insert(&app_state.db, new_books.into_inner()).await
}

pub async fn get_books(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, BookError> {
    db_read_books(&app_state.db).await
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
    db_delete_book_by_id(id, &app_state.db).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use chrono::Utc;

    const BOOK_ID0: i32 = 100;
    const BOOK_TITLE0: &str = "Unit Test title 0";
    const BOOK_AUTHOR0: &str = "Unit Test author 0";
    const BOOK_ID1: i32 = 101;
    const BOOK_TITLE1: &str = "Unit Test title 1";
    const BOOK_AUTHOR1: &str = "Unit Test author 1";
    const BOOK_ID2: i32 = 102;
    const BOOK_TITLE2: &str = "Unit Test title 2";
    const BOOK_AUTHOR2: &str = "Unit Test author 2";
    const BOOK_ID3: i32 = 103;
    const BOOK_TITLE3: &str = "Unit Test title 3";
    const BOOK_AUTHOR3: &str = "Unit Test author 3";
    const BOOK_ID4: i32 = 1;
    const BOOK_ID5: i32 = 2;
    const BOOK_ID6: i32 = 3;

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
            id: BOOK_ID0,
            title: BOOK_TITLE0.into(),
            author: BOOK_AUTHOR0.into(),
            posted_time: Some(Utc::now().naive_utc()),
        };

        let json_new_book = web::Json(new_book);
        let http_response = post_add_book(json_new_book, shared_data).await.unwrap();
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

        let new_book1 = Book {
            id: BOOK_ID1,
            title: BOOK_TITLE1.into(),
            author: BOOK_AUTHOR1.into(),
            posted_time: Some(Utc::now().naive_utc()),
        };

        let new_book2 = Book {
            id: BOOK_ID2,
            title: BOOK_TITLE2.into(),
            author: BOOK_AUTHOR2.into(),
            posted_time: Some(Utc::now().naive_utc()),
        };

        let new_book3 = Book {
            id: BOOK_ID3,
            title: BOOK_TITLE3.into(),
            author: BOOK_AUTHOR3.into(),
            posted_time: Some(Utc::now().naive_utc()),
        };

        let book_stack: Vec<Book> = Vec::from([new_book1, new_book2, new_book3]);

        let json_book_stack = web::Json(book_stack);
        let http_response = post_bulk_insert(json_book_stack, shared_data).await.unwrap();
        assert_eq!(http_response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_books() {
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

        let http_response = get_books(shared_data).await.unwrap();
        assert_eq!(http_response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_book_by_id() {
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

        let param: web::Path<i32> = web::Path::from(BOOK_ID4);
        let http_response = get_book_by_id(param, shared_data).await.unwrap();
        assert_eq!(http_response.status(), StatusCode::OK);
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
            id: BOOK_ID5,
            title: "Unit Test title updated".into(),
            author: "Unit Test author updated".into(),
            posted_time: Some(Utc::now().naive_utc()),
        };
        let param: web::Path<i32> = web::Path::from(updated_book.id);
        let json_updated_book = web::Json(updated_book);
        let http_response = put_book_by_id(param, json_updated_book, shared_data).await.unwrap();
        assert_eq!(http_response.status(), StatusCode::OK);
    }

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

        let param: web::Path<i32> = web::Path::from(BOOK_ID6);
        let http_response = delete_book_by_id(param, shared_data).await.unwrap();
        assert_eq!(http_response.status(), StatusCode::OK);
    }
}