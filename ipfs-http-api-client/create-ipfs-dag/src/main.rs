use std::io;
use std::io::{Cursor, Write};
use futures::TryStreamExt;
use ipfs_api_backend_actix::{IpfsApi, IpfsClient, TryFromUri};
use ipfs_api_backend_actix::request::{DagCodec, DagPut};


static IPFS_API: &str = "http://demo:32546/";
static CID_BOOK1: &str = "QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL";
static CID_BOOK2: &str = "QmYqo1Ack8g2rDX6TEoPA14oNASJrXEVB4oTEKv8So6Ect";
static CID_BOOK3: &str = "QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u";

#[actix_rt::main]
async fn main() {
    let client = IpfsClient::from_str(&IPFS_API)
        .unwrap();

//r#"[{"/":"QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL","name":[{"/":"QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL"}]}]"#
    let dag_node = Cursor::new(r#"[{ "/":"QmTN78XgBo6fPaWrDhsPf6yzJkcuqpEUBqVRtHu3i5yosL"},{ "/":"QmYqo1Ack8g2rDX6TEoPA14oNASJrXEVB4oTEKv8So6Ect"},{"/":"QmUfV4m2PUM559LSvDsJkoz1KofTVq25RDXwW5uMdjNb4u"}]"#);

    let response = client
        .dag_put(dag_node)
        .await
        .expect("error adding dag node");

    let cid = response.cid.cid_string;

    println!("CID:{}", cid);

    match client
        .dag_get(&cid)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await
    {
        Ok(bytes) => {
            println!("{}", String::from_utf8_lossy(&bytes[..]));
        }
        Err(e) => {
            eprintln!("error reading dag node: {}", e);
        }
    }
}