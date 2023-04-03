use std::fmt::Debug;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    #[serde(alias = "bytes")]
    pub bytes: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DocumentBytes {
    #[serde(alias = "/")]
    pub name: Content,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    #[serde(alias = "Sequence")]
    pub sequence: i64,
    #[serde(alias = "TTL")]
    pub ttl: i64,
    #[serde(alias = "Validity")]
    pub validity: DocumentBytes,
    #[serde(alias = "ValidityType")]
    pub validity_type: i32,
    #[serde(alias = "Value")]
    pub value: DocumentBytes,
}

#[derive(Serialize, Deserialize, Debug)]
struct Validation {
    #[serde(alias = "PublicKey")]
    pub public_key: String,
    #[serde(alias = "Reason")]
    pub reason: String,
    #[serde(alias = "Valid")]
    pub valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    #[serde(alias = "Data")]
    //pub data: String,
    pub data: Data,
    #[serde(alias = "PublicKey")]
    pub public_key: String,
    #[serde(alias = "Sequence")]
    pub sequence: i64,
    #[serde(alias = "SignatureV1")]
    pub signature_v1: String,
    #[serde(alias = "SignatureV2")]
    pub signature_v2: String,
    #[serde(alias = "TTL")]
    pub ttl: i64,
    #[serde(alias = "Validity")]
    pub validity: String,
    #[serde(alias = "ValidityType")]
    pub validity_type: i32,
    #[serde(alias = "Value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IpnsRecord {
    #[serde(alias = "Entry")]
    pub entry: Entry,
    //pub entry: String,
    #[serde(alias = "Validation")]
    pub validation: Option<Validation>,
}

async fn handle_name_inspect(api_server:String,verify:String,name:String,path:String) -> Result<IpnsRecord, Error> {
    let base_address = api_server + "/api/v0/name/inspect?verify=";
    let url = base_address + verify.as_str();

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
    let res = handle_name_inspect("http://demo:32546".to_string(), "/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j".to_string(), "signed-ipns-record.bin".to_string(), "signed-ipns-record.bin".to_string());
    let ipns_record = res.await.unwrap();
    match ipns_record.validation {
        Some(item) => {
            println!("Public key:{}", item.public_key);
            println!("Reason:{}", item.reason);
            println!("Valid:{}", item.valid);
        },
        None => { println!("Empty"); },
    }
}
