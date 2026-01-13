use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:5001").unwrap();
  println!("Echo server listening on 127.0.0.1:5001\nWaiting for connections...");

  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        handle_connection(&mut stream);
      }
      Err(e) => {
        eprintln!("Connection failed: {}", e);
      }
    }
  }
}

fn handle_connection(stream: &mut TcpStream) {
  // We clone the stream so we have one for reading and one for writing
  let reader_stream = stream.try_clone().expect("Failed to clone stream");
  let reader = BufReader::new(reader_stream);

  for line in reader.lines() {
    match line {
      Ok(content) => {
        // Echo the line back to the client
        // We add \n because reader.lines() strips it
        if let Err(e) = stream.write_all(format!("{}\n", content).as_bytes()) {
          eprintln!("Failed to write to stream: {}", e);
          break;
        }
        // Ensure the data is sent immediately
        stream.flush().unwrap();
      }
      Err(_) => break, // Connection closed or error
    }
  }
}
