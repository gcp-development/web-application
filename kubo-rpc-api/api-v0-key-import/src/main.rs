use std::fmt::Debug;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    #[serde(alias = "Id")]
    pub id: String,
    #[serde(alias = "Name")]
    pub name: String,
}

async fn handle_key_import(api_server:String,arg:String,name:String,path:String) -> Result<Key, Error> {
    let base_address = api_server + "/api/v0/key/import?arg=";
    let url = base_address + arg.as_str();

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
            let key_obj: Key = serde_json::from_str(&body_str).unwrap();
            Ok(key_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:file not uploaded.")),
    }
}

#[actix_web::main]
async fn main() {
    let res = handle_key_import("http://demo:32546".to_string(), "book3.json".to_string(), "private-key-ipns-record.bin".to_string(), "private-key-ipns-record.bin".to_string());
    let key = res.await.unwrap();
    println!("Id:{}", key.id);
    println!("Name:{}", key.name);
}