use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CID {
    #[serde(rename = "Path")]
    pub path: String,
}

async fn handle_name_resolve(api_server:String,arg:String) -> Result<CID, Error> {
    let base_address = api_server + "/api/v0/name/resolve?arg=";
    let url = base_address + arg.as_str();

    let client = Client::new();

    let mut response = client
        .post(url)
        .timeout(Duration::from_secs(120))
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {
            let body_bytes = response.body().await.unwrap();
            let body_str = std::str::from_utf8(&body_bytes).unwrap();
            let cid_obj: CID = serde_json::from_str(&body_str).unwrap();
            Ok(cid_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:route not created.")),
    }
}

#[actix_web::main]
async fn main() {
    let res = handle_name_resolve("http://demo:32546".to_string(), "/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j".to_string());
    let cid = res.await.unwrap();
    println!("Path:{}", cid.path);
}
