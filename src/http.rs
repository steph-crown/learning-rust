use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub enum HttpMethod {
  POST,
  GET,
  HEAD,
  PUT,
  DELETE,
  CONNECT,
  OPTIONS,
  TRACE,
  PATCH,
}

#[derive(Debug)]
pub struct Request {
  method: HttpMethod,
  path: String,
  version: String,
  headers: HashMap<String, String>,
  body: String,
}
// GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Mozilla\r\n\r\n
impl Request {
  pub fn new(str: &str) -> Option<Request> {
    parse_request(str)
  }

  pub fn method(&self) -> &HttpMethod {
    &self.method
  }

  pub fn path(&self) -> &str {
    &self.path
  }

  pub fn header(&self, key: &str) -> Option<&String> {
    self.headers.get(&key.to_lowercase())
  }
}

impl FromStr for HttpMethod {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "POST" => Ok(HttpMethod::POST),
      "GET" => Ok(HttpMethod::GET),
      "HEAD" => Ok(HttpMethod::HEAD),
      "PUT" => Ok(HttpMethod::PUT),
      "DELETE" => Ok(HttpMethod::DELETE),
      "CONNECT" => Ok(HttpMethod::CONNECT),
      "OPTIONS" => Ok(HttpMethod::OPTIONS),
      "TRACE" => Ok(HttpMethod::TRACE),
      "PATCH" => Ok(HttpMethod::PATCH),
      _ => Err(()),
    }
  }
}

pub fn parse_request(str: &str) -> Option<Request> {
  let mut lines = str.split("\r\n");
  let mut start_line = lines.next().map(|line| line.split_whitespace())?;

  let headers_iter: Vec<&str> = lines
    .by_ref()
    .take_while(|&line| !line.is_empty())
    .collect();
  let body = lines.collect::<Vec<&str>>().join("\r\n");

  let method: HttpMethod = start_line.next()?.parse().ok()?;
  let path = start_line.next()?.to_owned();
  let version = start_line.next()?.to_owned();

  let mut headers: HashMap<String, String> = HashMap::new();
  for header in headers_iter {
    if let Some((key, value)) = header.split_once(":") {
      headers.insert(key.to_lowercase(), value.trim().to_owned());
    }
  }

  Some(Request {
    method,
    path,
    version,
    headers,
    body,
  })
}
