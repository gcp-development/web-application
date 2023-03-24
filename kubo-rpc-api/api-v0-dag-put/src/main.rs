use std::fmt::Debug;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dag {
    #[serde(alias = "Cid")]
    pub cid: Record,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    #[serde(alias = "/")]
    pub cid_string: String,
}

async fn handle_add_dag(api_server:String,name:String,path:String) -> Result<Dag, Error> {

    let url=api_server+ "/api/v0/dag/put";

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
            let dag_obj: Dag = serde_json::from_str(&body_str).unwrap();
            Ok(dag_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:file not uploaded.")),
    }
}

#[actix_web::main]
async fn main() {
    let res = handle_add_dag("http://demo:32546".to_string(), "library.json".to_string(), "library.json".to_string());
    let dag = res.await.unwrap();
    println!("Cid:{}", dag.cid.cid_string);
}
