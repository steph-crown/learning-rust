use std::{
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
};

fn main() {
  shared_counter();
}

fn shared_counter() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles: Vec<JoinHandle<()>> = Vec::new();

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      for _ in 0..1000 {
        let mut num = counter.lock().unwrap();
        *num += 1;
      }
      // counter += 1;
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  // let num = *counter;
  println!("total {}", *counter.lock().unwrap())
}
