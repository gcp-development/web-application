use actix_web::HttpResponse;
use chrono::Utc;
use sqlx::postgres::PgPool;
use crate::model::book::Book;
use crate::errors::ServiceError;

pub async fn db_add_book(pool: &PgPool, book: Book) -> Result<HttpResponse, ServiceError> {
    let query_result = sqlx::query!("INSERT INTO public.books(id, title, author) VALUES ( $1, $2, $3)",
        book.id,
        book.title,
        book.author)
        .execute(pool)
        .await?;

    if query_result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().json("Book inserted"))
    } else {
        Err(ServiceError::NotFound("Book not inserted.".into(), ))
    }
}

pub async fn db_bulk_insert(pool: &PgPool, rows: Vec<Book>) -> Result<HttpResponse, ServiceError> {
    let mut book_id: Vec<i32> = Vec::new();
    let mut book_title: Vec<String> = Vec::new();
    let mut book_author: Vec<String> = Vec::new();
    rows.into_iter().for_each(|book| {
        book_id.push(book.id);
        book_title.push(book.title.into());
        book_author.push(book.author.into());
    });

    let query_result = sqlx::query!("INSERT INTO public.books(id, title, author) SELECT * FROM UNNEST ($1::int4[], $2::text[], $3::text[])",
        &book_id[..],
        &book_title[..],
        &book_author[..]
    )
        .execute(pool)
        .await?;
    if query_result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().json("Books inserted"))
    } else {
        Err(ServiceError::NotFound("Books not inserted.".into(), ))
    }
}

pub async fn db_read_books(pool: &PgPool) -> Result<Vec<Book>, ServiceError> {
    let query_rows = sqlx::query!("SELECT id, title, author, record_timestamp FROM public.books;")
        .fetch_all(pool)
        .await
        .unwrap();

    let query_result: Vec<Book> = query_rows
        .iter()
        .map(|row| Book {
            id: row.id,
            title: row.title.clone(),
            author: row.author.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(row.record_timestamp.unwrap())),
        })
        .collect();
    if query_result.len() > 0 {
        Ok(query_result)
    } else {
        Err(ServiceError::NotFound("The library has no books.".into(), ))
    }
}

pub async fn db_read_book_by_id(id: i32, pool: &PgPool) -> Result<Book, ServiceError> {
    let query_row = sqlx::query!("SELECT id, title, author, record_timestamp FROM public.books WHERE id = $1",id)
        .fetch_all(pool)
        .await
        .unwrap();

    if query_row.len() > 0 {
        Ok(Book {
            id: query_row[0].id,
            title: query_row[0].title.clone(),
            author: query_row[0].author.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(query_row[0].record_timestamp.unwrap())),
        })
    } else {
        Err(ServiceError::NotFound("Book not found.".into(), ))
    }
}

pub async fn db_update_book_by_id(id: i32, updated_book: Book, pool: &PgPool) -> Result<Book, ServiceError> {
    let timestamp = Some(Utc::now().naive_utc());
    let query_result = sqlx::query!("UPDATE books SET title= $2, author= $3, record_timestamp = $4 WHERE id = $1;",
        id,
        updated_book.title,
        updated_book.author,
        timestamp)
        .execute(pool)
        .await
        .unwrap();

    if query_result.rows_affected() > 0 {
        Ok(Book {
            id: updated_book.id,
            title: updated_book.title.clone(),
            author: updated_book.author.clone(),
            posted_time: timestamp,
        })
    } else {
        Err(ServiceError::NotFound("Book not updated.".into(), ))
    }
}

pub async fn db_delete_book_by_id(id: i32, pool: &PgPool) -> Result<HttpResponse, ServiceError> {
    let query_result = sqlx::query!("DELETE FROM books WHERE id = $1;", id)
        .execute(pool)
        .await
        .unwrap();
    if query_result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().json("Book deleted."))
    } else {
        Err(ServiceError::NotFound("Book not updated.".into(), ))
    }
}