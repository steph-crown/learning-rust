use std::num::NonZeroU32;

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

  println!("{:#?} {}", request, " ".is_empty());
}
