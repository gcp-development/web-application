use super::errors::BookError;
use chrono::Utc;
use super::model::Book;
use sqlx::postgres::PgPool;

pub async fn add_book(pool: &PgPool, book: Book) -> Result<Book, BookError> {
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


pub async fn bulk_insert(pool: &PgPool, rows: Vec<Book>) -> Result<bool, BookError> {
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

pub async fn get_books(pool: &PgPool) -> Result<Vec<Book>, BookError> {
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