use reqwest::header::{HeaderMap, HeaderValue};
use urlencoding::decode;

fn convert_headers(request: &tide::Request<()>) -> HeaderMap {
  let mut header_map = HeaderMap::new();

  for (name, value) in request.iter() {
      if let Ok(header_name) = reqwest::header::HeaderName::try_from(name.as_str()) {
          if let Ok(header_value) = reqwest::header::HeaderValue::from_str(value.as_str()) {
              header_map.insert(header_name, header_value);
          }
      }
  }

  // Remove headers that would cause issues, since this is proxying requests
  header_map.remove("host");
  header_map.remove("accept-encoding");

  header_map
}

/**
 * Literally all this does is receive requests with a "url" param, and return the result of a GET request to that URL.
 */
pub async fn start_server(port: u16) {
  let mut app = tide::new();

  app.at("/url").get(handle_request);
  app.at("/url").post(handle_request);
  app.at("/url").put(handle_request);
  app.at("/url").delete(handle_request);

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

  let req_body = req.body_bytes().await.unwrap_or(vec![]);
  let req_method = req.method().to_string();

  // Convert tide request headers to reqwest headers
  let req_headers = convert_headers(&req);

  let response = match req_method.as_str() {
    "GET" => do_get(url.to_string(), req_headers),
    "POST" => do_post(url.to_string(), req_body, req_headers),
    "PUT" => do_put(url.to_string(), req_body, req_headers),
    "DELETE" => do_delete(url.to_string(), req_body, req_headers),
    _ => return Ok(tide::Response::new(400))
  };

  let headers = response.headers().clone();
  let mut res = tide::Response::new(response.status().as_u16());
  let bytes = response.bytes().unwrap().to_vec();

  // Set content type to text if none is provided
  let default_content_type = &HeaderValue::from_static("text/plain");
  let content_type = headers.get("content-type").unwrap_or(default_content_type);
  
  res.set_body(bytes);
  res.set_content_type(content_type.to_str().unwrap());
  res.insert_header("Access-Control-Allow-Origin", "*");

  Ok(res)
}

pub fn do_get(url: String, headers: HeaderMap) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .get(url)
    .headers(headers)
    .send()
    .unwrap()
}

pub fn do_post(url: String, body: Vec<u8>, headers: HeaderMap) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .post(url)
    .body(body)
    .headers(headers)
    .send()
    .unwrap()
}

pub fn do_put(url: String, body: Vec<u8>, headers: HeaderMap) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .put(url)
    .body(body)
    .headers(headers)
    .send()
    .unwrap()
}

pub fn do_delete(url: String, body: Vec<u8>, headers: HeaderMap) -> reqwest::blocking::Response {
  reqwest::blocking::Client::new()
    .delete(url)
    .body(body)
    .headers(headers)
    .send()
    .unwrap()
}