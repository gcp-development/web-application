use std::fmt::Debug;
use std::ops::Add;
use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

use prost::{self, Enumeration, Message};
use chrono::{Duration, SecondsFormat, Utc};
use strum::Display;

#[derive(
Clone,
Copy,
Debug,
PartialEq,
Eq,
Hash,
PartialOrd,
Ord,
Enumeration,
Display,
Serialize,
Deserialize,
)]
#[repr(i32)]
pub enum ValidityType {
    EOL = 0,
}

#[actix_web::main]
async fn main() {
    /*
    let params = [("arg","/ipns/12D3KooWLTSofM6cdCNGjQ5Rnid1EN3Vyorcb2M6QoBsEPyf1Y6a")];

    let url = "http://demo:32546/api/v0/routing/put";

    let client = Client::new();
    let mut form = multipart::Form::default();

    form.add_file("library.json", "library.json").unwrap();

    let mut response = client
        .post(url)
        .content_type(form.content_type())
        //.send_form(&params)
        .send_body(multipart::Body::from(form))
        .await
        .unwrap();

    let body_bytes = response.body().await.unwrap();
    let body_str = std::str::from_utf8(&body_bytes).unwrap();
    println!("Body:{}", body_str);
    //let dag_obj: Dag = serde_json::from_str(&body_str).unwrap();
    //println!("Cid:{}",dag_obj.cid.cid_string);
    //   let result = body_str.replace("Body:b", "");
    // let book_obj: Book = serde_json::from_str(&result).unwrap();

*/
    let valid_for = chrono::Duration::days(364);
    //let b = Duration::minutes(55);
    let signer="k51qzi5uqu5dg88xo6rqurksk3jbg5bkql8y5k04q1tt8t2ld4cus6l5oc50o8";
    let cid = "QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL";
    let value = format!("/ipfs/{}", cid.to_string()).into_bytes();

    let validity = chrono::Utc::now()
        .add(valid_for)
        .to_rfc3339_opts(SecondsFormat::Nanos, false)
        .into_bytes();

    let validity_type = ValidityType::EOL;

    let signing_input_v1 = {
        let mut data = Vec::with_capacity(
            value.len() + validity.len() + 3, /* b"EOL".len() == 3 */
        );

        data.extend(value.iter());
        data.extend(validity.iter());
        data.extend(validity_type.to_string().as_bytes());

        data
    };

    let mut pub_key = signer.crypto_key().encode_to_vec(); // Protobuf encoding

    if pub_key.len() <= 42 {
        pub_key.clear();
    }

    let signature_v1 = signer.try_sign(&signing_input_v1)?;
    let signature_v1 = signature_v1.as_bytes().to_vec();
}
