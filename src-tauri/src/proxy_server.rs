use reqwest::header::{HeaderMap, HeaderValue};
use tide::{self, http::headers::HeaderValues};
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

    match method.as_str() {
      "GET" => do_get(url.to_string(), req.header("Authorization")),
      "POST" => do_post(url.to_string(), req.body_bytes().await.unwrap_or(vec![]), req.header("Authorization")),
      "PUT" => do_put(url.to_string(), req.body_bytes().await.unwrap_or(vec![]), req.header("Authorization")),
      _ => return Ok(tide::Response::new(400))
    }
  };
  let headers = response.headers().clone();
  let mut res = tide::Response::new(response.status().as_u16());
  let bytes = response.bytes().unwrap().to_vec();
  
  res.set_body(bytes);
  res.set_content_type(headers.get("content-type").unwrap().to_str().unwrap());
  res.insert_header("Access-Control-Allow-Origin", "*");

  Ok(res)
}

pub fn do_get(url: String, auth: Option<&HeaderValues>) -> reqwest::blocking::Response {
  let mut headers = HeaderMap::new();

  if let Some(auth) = auth {
    let value = auth.as_str();
    let auth = HeaderValue::from_str(value).unwrap();

    headers.insert("Authorization", auth);
  }

  reqwest::blocking::Client::new()
    .get(url)
    .headers(headers)
    .send()
    .unwrap()
}

pub fn do_post(url: String, body: Vec<u8>, auth: Option<&HeaderValues>) -> reqwest::blocking::Response {
  let mut headers = HeaderMap::new();

  if let Some(auth) = auth {
    let value = auth.as_str();
    let auth = HeaderValue::from_str(value).unwrap();

    headers.insert("Authorization", auth);
  }

  reqwest::blocking::Client::new()
    .post(url)
    .body(body)
    .headers(headers)
    .send()
    .unwrap()
}

pub fn do_put(url: String, body: Vec<u8>, auth: Option<&HeaderValues>) -> reqwest::blocking::Response {
  let mut headers = HeaderMap::new();

  if let Some(auth) = auth {
    let value = auth.as_str();
    let auth = HeaderValue::from_str(value).unwrap();

    headers.insert("Authorization", auth);
  }

  reqwest::blocking::Client::new()
    .put(url)
    .body(body)
    .headers(headers)
    .send()
    .unwrap()
}