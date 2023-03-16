use std::io::Cursor;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};

static IPFS_API: &str = "http://demo:32546/";

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::from_str(&IPFS_API)
        .unwrap();
    let data = Cursor::new("Hello World!");

    match client.version().await {
        Ok(version) => println!("version: {}", version.version),
        Err(e) => eprintln!("error getting version: {}", e),
    }
    /*
    match client.add(data).await {
        Ok(res) => println!("{}", res.hash),
        Err(e) => eprintln!("error adding file: {}", e)
    }
    */

}
