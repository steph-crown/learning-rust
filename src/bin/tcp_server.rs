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
  Unknown,
}

impl Command {
  fn from_str(s: &str) -> Command {
    match s {
      "ECHO" => Command::ECHO,
      "UPPERCASE" => Command::UPPERCASE,
      "REVERSE" => Command::REVERSE,
      "QUIT" => Command::QUIT,
      _ => Command::Unknown,
    }
  }
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
  // let output: String;

  let mut tokens = content.split(" ");
  let first = tokens.next().unwrap_or("");
  let rest = tokens.collect::<Vec<&str>>().join(" ");
  let mut quit = false;
  let command = Command::from_str(first);

  let output: String = match command {
    Command::ECHO => rest.to_string(),
    Command::UPPERCASE => rest.to_uppercase(),
    Command::REVERSE => rest.chars().rev().collect(),
    Command::QUIT => {
      quit = true;
      "Goodbye!".to_string()
    }
    Command::Unknown => {
      "Incorrect command; Enter one of ECHO, UPPERCASE, REVERSE, QUIT".to_string()
    }
  };

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
