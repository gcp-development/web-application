use actix_web::HttpResponse;
use sqlx::{Execute, Executor, Row};
use super::errors::BookError;
use super::model::Book;
use sqlx::postgres::PgPool;

pub async fn db_add_book(pool: &PgPool, book: Book) -> Result<Book, BookError> {
    let row = sqlx::query!("INSERT INTO public.books(id, title, author) VALUES ( $1, $2, $3)
    returning
    id,
    title,
    author",
    book.id,
    book.title,
    book.author
    )
        .fetch_one(pool)
        .await?;
    Ok(Book {
        id: row.id,
        title: row.title.clone(),
        author: row.author.clone(),
        posted_time: Some(book.posted_time.unwrap()),
    })
}

pub async fn db_bulk_insert(pool: &PgPool, rows: Vec<Book>) -> Result<bool, BookError> {
    let mut book_id: Vec<i32> = Vec::new();
    let mut book_title: Vec<String> = Vec::new();
    let mut book_author: Vec<String> = Vec::new();
    rows.into_iter().for_each(|book| {
        book_id.push(book.id);
        book_title.push(book.title.into());
        book_author.push(book.author.into());
    });

    let todo = sqlx::query!("INSERT INTO public.books(id, title, author) SELECT * FROM UNNEST ($1::int4[], $2::text[], $3::text[])",
        &book_id[..],
    &book_title[..],
    &book_author[..]
    )
        .execute(pool)
        .await?;
    Ok(true)
}

pub async fn db_books(pool: &PgPool) -> Result<Vec<Book>, BookError> {
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
        Err(BookError::NotFound("The library has no books.".into(), ))
    }
}

pub async fn db_read_book_by_id(id: i32, pool: &PgPool) -> Result<Book, BookError> {
    let row = sqlx::query!("SELECT id, title, author, record_timestamp FROM public.books WHERE id = $1",id)
        .fetch_all(pool)
        .await
        .unwrap();

    if row.len() > 0 {
        Ok(Book {
            id: row[0].id,
            title: row[0].title.clone(),
            author: row[0].author.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(row[0].record_timestamp.unwrap())),
        })
    } else {
        Err(BookError::NotFound("Book not found.".into(), ))
    }
}

pub async fn db_update_book_by_id(id: i32, updated_book: Book, pool: &PgPool) -> Result<Book, BookError> {
    let row = sqlx::query!("UPDATE books SET title= $2, author= $3, record_timestamp = $4 WHERE id = $1;",
        updated_book.id,
        updated_book.title,
        updated_book.author,
        updated_book.posted_time)
        .fetch_all(pool)
        .await
        .unwrap();

    if row.len() > 0 {
        Ok(Book {
            id: updated_book.id,
            title: updated_book.title.clone(),
            author: updated_book.author.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(updated_book.posted_time.unwrap())),
        })
    } else {
        Err(BookError::NotFound("Book not updated.".into(), ))
    }
}

pub async fn db_delete_book_by_id(id: i32, pool: &PgPool) -> Result<String, BookError> {
    let queryResult = sqlx::query!("DELETE FROM books WHERE id = $1;",
        id)
        .fetch_one(pool)
        .await
        .unwrap();
    if queryResult.is_empty() == true {
        Ok("Book deleted.".to_string())
    } else {
        Err(BookError::NotFound("Book not updated.".into(), ))
    }
}