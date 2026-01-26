use std::{
  fs,
  io::{BufReader, prelude::*},
  net::{TcpListener, TcpStream},
  thread,
  time::Duration,
};

use prog_concepts::ThreadPool;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
  println!("Listening on port 7878");

  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream = stream.unwrap();

    // handle_connection(stream);
    // thread::spawn(|| {
    //   handle_connection(stream);
    // });

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&stream);

  let request_line = match buf_reader.lines().next() {
    Some(Ok(line)) => line,
    _ => {
      // Handle error: send 400 Bad Request
      let response = "HTTP/1.1 400 BAD REQUEST\r\n\r\n";
      stream.write_all(response.as_bytes()).unwrap();
      return;
    }
  };

  let (status_line, filename) = match &request_line[..] {
    "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    "GET /sleep HTTP/1.1" => {
      thread::sleep(Duration::from_secs(5));
      ("HTTP/1.1 200 OK", "hello.html")
    }
    _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
  };

  let contents = fs::read_to_string(filename).unwrap();
  let length = contents.len();

  let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

  stream.write_all(response.as_bytes()).unwrap();
}
