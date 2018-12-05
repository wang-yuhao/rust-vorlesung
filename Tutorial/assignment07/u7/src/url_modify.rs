extern crate hyper;
use std::str::from_utf8;
use hyper::{Client, Uri};
use hyper::rt::{self, Future, Stream};
fn main() {
let url = "http://httpbin.org/ip".parse().unwrap();
rt::run(fetch(url));
}

fn fetch(url: Uri) -> impl Future<Item=(), Error=()> {
let client = Client::new();
client.get(url)
.and_then(|res: hyper::Response<_>| {
println!("Response: {}", res.status());
res.into_body().concat2() })
.and_then(|chunk: hyper::Chunk| {
println!("Body follows >>>\n{}<<<",
from_utf8(&chunk).unwrap());
Ok(()) }) // need to return a () future
.map_err(|err| println!("Error: {}", err)) }