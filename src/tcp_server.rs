use std::{
  io::{BufRead, BufReader, Write},
  net::{TcpListener, TcpStream},
};

// --- Types and Enums ---

#[derive(PartialEq)]
enum ControlFlow {
  Break,
  Continue,
}

enum Command {
  Echo,
  Uppercase,
  Reverse,
  Quit,
  Unknown,
}

impl Command {
  fn from_str(s: &str) -> Self {
    match s {
      "ECHO" => Command::Echo,
      "UPPERCASE" => Command::Uppercase,
      "REVERSE" => Command::Reverse,
      "QUIT" => Command::Quit,
      _ => Command::Unknown,
    }
  }
}

// --- Server Implementation ---

pub struct TcpServer {
  address: String,
}

impl TcpServer {
  pub fn new(addr: &str) -> Self {
    Self {
      address: addr.to_string(),
    }
  }

  /// Starts the server loop
  pub fn run(&self) {
    let listener = TcpListener::bind(&self.address).expect("Could not bind to address");
    println!(
      "Echo server listening on {}\nWaiting for connections...",
      self.address
    );

    for stream in listener.incoming() {
      match stream {
        Ok(mut stream) => {
          println!("New connection: {:?}", stream.peer_addr().unwrap());
          self.handle_connection(&mut stream);
        }
        Err(e) => {
          eprintln!("Connection failed: {}", e);
        }
      }
    }
  }

  fn handle_connection(&self, stream: &mut TcpStream) {
    // Clone for reading while keeping the original for writing
    let reader_stream = stream.try_clone().expect("Failed to clone stream");
    let reader = BufReader::new(reader_stream);

    for line in reader.lines() {
      match line {
        Ok(content) => {
          if self.process_line(&content, stream) == ControlFlow::Break {
            break;
          }
        }
        Err(_) => break, // Connection closed by client
      }
    }
    println!("Connection closed.");
  }

  fn process_line(&self, content: &str, stream: &mut TcpStream) -> ControlFlow {
    // Using splitn(2, ' ') ensures we get the command and the exact "rest"
    // including all original whitespaces.
    let mut tokens = content.splitn(2, ' ');
    let first = tokens.next().unwrap_or("");
    let rest = tokens.next().unwrap_or("");

    let command = Command::from_str(first);
    let mut should_quit = false;

    let output = match command {
      Command::Echo => rest.to_string(),
      Command::Uppercase => rest.to_uppercase(),
      Command::Reverse => rest.chars().rev().collect(),
      Command::Quit => {
        should_quit = true;
        "Goodbye!".to_string()
      }
      Command::Unknown => {
        "Incorrect command; Enter one of ECHO, UPPERCASE, REVERSE, QUIT".to_string()
      }
    };

    // Write the result back to the client
    if let Err(e) = stream.write_all(format!("{}\n", output).as_bytes()) {
      eprintln!("Write error: {}", e);
      return ControlFlow::Break;
    }

    // Ensure data is sent immediately
    let _ = stream.flush();

    if should_quit {
      ControlFlow::Break
    } else {
      ControlFlow::Continue
    }
  }
}
