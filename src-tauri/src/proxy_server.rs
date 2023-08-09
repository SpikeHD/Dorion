use std::str::FromStr;

use reqwest::header::HeaderMap;
use tide::{self, http::headers::HeaderValue};
use urlencoding::decode;

/**
 * Literally all this does is receive requests with a "url" param, and return the result of a GET request to that URL.
 */
pub async fn start_server(port: u16) {
  let mut app = tide::new();

  app.at("/url").get(handle_request);
  app.at("/url").post(handle_request);
  app.at("/url").put(handle_request);

  println!("Proxy server listening on port {}", port);

  app.listen(format!("127.0.0.1:{}", port)).await.unwrap();
}

pub async fn handle_request(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
  let full_param = decode(req.url().query().unwrap_or("")).unwrap().into_owned();
  let url = full_param.split("url=").last().unwrap_or("");

  if url.is_empty() {
    println!("Got an empty URL");
    return Ok(tide::Response::new(400));
  }

  let response = {
    let method = req.method().to_string();

    println!("Got method: {}", method);

    match method.as_str() {
      "GET" => do_get(url.to_string()),
      "POST" => do_post(url.to_string(), req.body_bytes().await.unwrap_or(vec![])),
      "PUT" => do_put(url.to_string(), req.body_bytes().await.unwrap_or(vec![])),
      _ => return Ok(tide::Response::new(400))
    }
  };
  let headers = response.headers().clone();
  let mut res = tide::Response::new(response.status().as_u16());
  let bytes = response.bytes().unwrap().to_vec();
  
  res.set_body(bytes);
  res.set_content_type(headers.get("content-type").unwrap().to_str().unwrap());
  res.insert_header("Access-Control-Allow-Origin", HeaderValue::from_str("*").unwrap());

  Ok(res)
}

pub fn do_get(url: String) -> reqwest::blocking::Response {
  reqwest::blocking::get(url).unwrap()
}

pub fn do_post(url: String, body: Vec<u8>) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .post(url)
    .body(body)
    .send()
    .unwrap()
}

pub fn do_put(url: String, body: Vec<u8>) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .put(url)
    .body(body)
    .send()
    .unwrap()
}