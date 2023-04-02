use actix_multipart_rfc7578::client::{multipart};
use awc::{Client};
use actix_web::{error,Error,Result};
use actix_web::http::StatusCode;
use serde::{Deserialize, Deserializer, Serialize};
use std::time::Duration;



#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response {
    #[serde(rename = "Addrs", deserialize_with = "nullable_vector")]
    pub multi_addresses: Vec<String>,
    #[serde(rename = "ID")]
    pub peer_id: String,
}

fn nullable_vector<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    match opt {
        None => {
            let x= Vec::new();
            Ok(x)
        }
        Some(x) => {
            Ok(x)
        }
    }
    //Ok(opt.unwrap_or_else( Vec<serde_json::Value::String()>::new()))
}

#[derive(Serialize, Deserialize)]
struct Route {
    #[serde(rename = "Extra")]
    pub extra: String,
    #[serde(rename = "ID")]
    pub peer_id: String,
    #[serde(rename = "Responses")]
    pub responses: Vec<Response>,
    #[serde(rename = "Type")]
    pub r#type: i32,
}

async fn handle_routing_put(api_server:String,arg:String,name:String,path:String) -> Result<Route, Error> {
    let base_address = api_server + "/api/v0/routing/put?arg=";
    let url = base_address + arg.as_str();

    let client = Client::new();
    let mut form = multipart::Form::default();

    form.add_file(name, path).unwrap();

    let mut response = client
        .post(url)
        .timeout(Duration::from_secs(120))
        .content_type(form.content_type())
        .send_body(multipart::Body::from(form))
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => {
            let body_bytes = response.body().await.unwrap();
            let body_str = std::str::from_utf8(&body_bytes).unwrap();
            println!("{}",body_str);
            let route_obj: Route = serde_json::from_str(&body_str).unwrap();
            Ok(route_obj)
        },
        _ => Err(error::ErrorInternalServerError("Error:route not created.")),
    }
}

#[actix_web::main]
async fn main() {
    let res = handle_routing_put("http://demo:32546".to_string(), "/ipns/k2k4r8lpp59iv154i7dfnd5m99tke25rqhqaybpssnk3ds5h5t5boe8j".to_string(), "signed-ipns-record.bin".to_string(), "signed-ipns-record.bin".to_string());
    let route = res.await.unwrap();
    println!("Extra:{}", route.extra);
    println!("Peer Id:{}", route.peer_id);
    println!("type:{}", route.r#type);
    match route.responses.len() > 1 {
        true => {
            for item1 in route.responses.iter() {
                println!("Peer Id: {}", item1.peer_id);
                //for item2 in item1.multi_addresses.iter() {
                //    println!("Multiaddress : {}", item2);
               // }
                /*match &item1.multi_addresses {
                    Some(vector) => {
                        for item2 in vector.iter() {
                            println!("Multiaddress : {}", item2);
                        }
                    },
                    None => {
                        println!("Empty");
                    }
                }*/
            }
        },
        _ => { println!("Empty"); },
    }
}
