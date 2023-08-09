use std::str::FromStr;

use tide::{self, http::headers::HeaderValue};
use urlencoding::decode;

/**
 * Literally all this does is receive requests with a "url" param, and return the result of a GET request to that URL.
 */
pub async fn start_server(port: u16) {
  let mut app = tide::new();

  app.at("/url").get(handle_request);

  println!("Proxy server listening on port {}", port);

  app.listen(format!("127.0.0.1:{}", port)).await.unwrap();
}

pub async fn handle_request(mut req: tide::Request<()>) -> Result<tide::Response, tide::Error> {
  let full_param = decode(req.url().query().unwrap_or("")).unwrap().into_owned();
  let url = full_param.split("url=").last().unwrap_or("");

  if url.is_empty() {
    return Ok(tide::Response::new(400));
  }

  let response = reqwest::blocking::get(url).unwrap();
  let headers = response.headers().clone();
  let bytes = response.bytes().unwrap().to_vec();
  
  let mut res = tide::Response::new(200);
  res.set_body(bytes);
  res.set_content_type(headers.get("content-type").unwrap().to_str().unwrap());
  res.insert_header("Access-Control-Allow-Origin", HeaderValue::from_str("*").unwrap());

  Ok(res)
}