use std::{collections::HashMap, num::NonZeroU32};

// mod collections;
// mod employees;
// mod pig_latin;
// mod generics;
mod http;
mod lifetimes;

fn main() {
  // crate::collections::run();
  // crate::pig_latin::run();
  // crate::employees::run();
  // crate::generics::run();
  // crate::lifetimes::run();

  let Some(request) = http::Request::new(
    "GET /index.html  HTTP/1.1\r\nHost: example.com\r\nUser-Agent: Mozilla\r\n\r\nname=FirstName+LastName&email=bsmth%40example.com",
  ) else {
    return ();
  };

  // let mut headers = HashMap::new();
  // headers.insert("Content-Type".to_string(), "application/json".to_string());
  // headers.insert("Sintent-Type".to_string(), "application/json".to_string());

  // let response = http::Response::new(http::StatusCode::Created, "body".to_string(), headers);

  // let response = http::Response::parse(&response);

  // println!("{:#?} {}", request, " ".is_empty());
  println!("{:#?}", request);
}
