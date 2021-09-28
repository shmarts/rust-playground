use hyper::body;
use hyper::{Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::Deserialize;
use std::str::from_utf8;

#[derive(Deserialize, Debug)]
struct User {
  name: String,
  location: String,
  followers: u32,
}

#[tokio::main]
async fn get() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let https = HttpsConnector::new();
  let client = Client::builder().build::<_, hyper::Body>(https);

  let req = Request::builder()
    .method(Method::GET)
    .uri("https://api.github.com/users/shmarts")
    .header("User-Agent", "shmarts/rust-playground")
    .body(Body::empty())?;

  let res = client.request(req).await?;

  let status = res.status();
  if status.is_client_error() || status.is_server_error() {
    panic!("got client or server error {}", status.as_u16());
  }

  let buf = body::to_bytes(res.into_body()).await?;
  let json = from_utf8(&buf).unwrap();

  let user: User = serde_json::from_str(&json).unwrap();
  println!("{:#?}", user);

  Ok(())
}

fn main() {
  get().unwrap();
}
