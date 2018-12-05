// extern crate hyper;
// use std::str::from_utf8;
// use hyper::{Client, Uri};
// use hyper::rt::{self, Future, Stream};
// fn main() {
// let url = "http://httpbin.org/ip".parse().unwrap();
//         rt::run(fetch(url));
// }

// fn fetch(url: Uri) -> impl Future<Item=(), Error=()> {
//     let client = Client::new();
//     client.get(url)
//         .and_then(|res: hyper::Response<_>| {
//     println!("Response: {}", res.status());
//     res.into_body().concat2() })
//          .and_then(|chunk: hyper::Chunk| {
//     println!("Body follows >>>\n{}<<<",
//     from_utf8(&chunk).unwrap());
//     Ok(()) }) // need to return a () future
//         .map_err(|err| println!("Error: {}", err)) }

// fn fetch_crateinfo(url: Uri) -> impl Future<Item=String, Error=()>




extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use futures::{future, Future};
use hyper_tls::HttpsConnector;
use hyper::Client;

fn main() {
    tokio::run(future::lazy(|| {
        // 4 is number of blocking DNS threads
        let https = HttpsConnector::new(4).unwrap();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);

        client
            .get("https://hyper.rs".parse().unwrap())
            .map(|res| {
                assert_eq!(res.status(), 200);
            })
            .map_err(|e| println!("request error: {}", e))
    }));
}


fn fetch_crateinfo(url:Url) -> impl Future<Item=Stream, Error=()>{
    //new HTTPS connection with 4 DNS worker threads
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder().build::<_, hyper::Body>(https);
    client
    .get(url)
    .and_then(|resp:hyper::Response<_>|{
        resp.into_body()
        .concat2()    })
    .map_err(|_| ())
    .and_then(|chunk:hyper::Chunk|{

        std::str::from_utf8(&chunk)
            .map(|s| s.to_string())
            .map_err(|_| ())    })
}