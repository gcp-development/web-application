use std::fmt::Debug;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Hash")]
    pub hash: String,
    #[serde(alias = "Size",deserialize_with = "deserialize_string_to_i32")]
    pub size: i32,
}

pub fn deserialize_string_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where D: Deserializer<'de> {
    let buf = String::deserialize(deserializer)?;
    buf.parse::<i32>().map_err(serde::de::Error::custom)
}

async fn handle_add_file(api_server:String,name:String,path:String) -> Result<Book, Error> {

    let url=api_server+ "/api/v0/add";

    let client = Client::new();
    let mut form = multipart::Form::default();

    form.add_file(name, path).unwrap();

    let mut response = client
        .post(url)
        .content_type(form.content_type())
        .send_body(multipart::Body::from(form))
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {
            let body_bytes = response.body().await.unwrap();
            let body_str = std::str::from_utf8(&body_bytes).unwrap();
            let result = body_str.replace("Body:b", "");
            let book_obj: Book = serde_json::from_str(&result).unwrap();
            Ok(book_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:file not uploaded.")),
    }
}

#[actix_web::main]
async fn main() {
    let res = handle_add_file("http://demo:32546".to_string(), "book3.json".to_string(), "book3.json".to_string());
    let book = res.await.unwrap();
    println!("Name:{:?}", book.name);
    println!("Hash:{:?}", book.hash);
    println!("Size:{:?}", book.size);
}