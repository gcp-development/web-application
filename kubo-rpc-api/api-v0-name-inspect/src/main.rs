use std::fmt::Debug;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Validation {
    pub public_key: String,
    pub reason: String,
    pub valid: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    pub data: String,
    pub public_key: String,
    pub sequence: String,
    pub signature_v1: String,
    pub signature_v2: String,
    pub ttl: String,
    pub validity: String,
    pub validity_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IpnsRecord {
    pub entry: Entry,
    pub validation: Validation,
}

async fn handle_inspect_ipns_record(api_server:String,name:String,path:String) -> Result<IpnsRecord, Error> {

    let url=api_server+ "/api/v0/name/inspect";

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
            let ipns_record_obj: IpnsRecord = serde_json::from_str(&body_str).unwrap();
            Ok(ipns_record_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:file not uploaded.")),
    }
}

#[actix_web::main]
async fn main() {

    let res = handle_inspect_ipns_record("http://demo:32546".to_string(), "IPNS_Record.txt".to_string(), "IPNS_Record.txt".to_string());
    let ipns_record = res.await.unwrap();
    println!("Public key:{}", ipns_record.validation.public_key);
    println!("Reason:{}", ipns_record.validation.reason);
    println!("Valid:{}", ipns_record.validation.valid);

    /*
    let url = "http://demo:32546/api/v0/name/inspect";

    let client = Client::new();
    let mut form = multipart::Form::default();

    form.add_file("IPNS_Record.txt", "IPNS_Record.txt").unwrap();

    let mut response = client
        .post(url)
        .content_type(form.content_type())
        .send_body(multipart::Body::from(form))
        .await
        .unwrap();

    let body_bytes = response.body().await.unwrap();
    let body_str = std::str::from_utf8(&body_bytes).unwrap();
    println!("Body:{}", body_str);
    //let dag_obj: Dag = serde_json::from_str(&body_str).unwrap();
    //println!("Cid:{}",dag_obj.cid.cid_string);
        // let book_obj: Book = serde_json::from_str(&result).unwrap();

     */
}
