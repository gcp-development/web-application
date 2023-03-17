use std::io;
use std::io::{ Write };
use futures::TryStreamExt;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};

static IPFS_API: &str = "http://demo:32546/";
static CID: &str = "QmYzyPxVtuZ1Vqby3NQHUEkWMjRq1nKuBGanazknDCnCvV";

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::from_str(&IPFS_API)
        .unwrap();

    match client
        .get(CID)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(res) => {
            let out = io::stdout();
            let mut out = out.lock();

            out.write_all(&res).unwrap();
        }
        Err(e) => eprintln!("error getting file: {}", e)
    }
}
