use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

#[derive(PartialEq)]
enum ControlFlow {
  Break,
  Continue,
}

enum Command {
  ECHO,
  UPPERCASE,
  REVERSE,
  QUIT,
}

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
  let reader_stream = stream.try_clone().expect("Failed to clone stream");
  let reader = BufReader::new(reader_stream);

  for line in reader.lines() {
    match line {
      Ok(content) => {
        if process_line(&content, stream) == ControlFlow::Break {
          break;
        }
      }
      Err(_) => break, // Connection closed or error
    }
  }
}

fn process_line(content: &str, stream: &mut TcpStream) -> ControlFlow {
  let mut output = String::new();

  let mut tokens = content.split_ascii_whitespace();
  let first = tokens.next();
  let rest = tokens.collect::<Vec<&str>>().join("");
  let mut quit = false;

  match first {
    Some("ECHO") => output = rest,
    Some("UPPERCASE") => output = rest.to_uppercase(),
    Some("REVERSE") => output = rest.chars().rev().collect(),
    Some("QUIT") => {
      output = "Goodbye!".to_string();
      quit = true;
    }
    _ => eprintln!("Error"),
  }

  if let Err(e) = stream.write_all(format!("{}\n", output).as_bytes()) {
    eprintln!("Failed to write to stream: {}", e);
    return ControlFlow::Break;
  }
  stream.flush().unwrap();

  if quit {
    return ControlFlow::Break;
  }
  ControlFlow::Continue
}
