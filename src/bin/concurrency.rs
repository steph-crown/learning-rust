use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
  let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
  let (_x, _y): (String, String);

  let handle = thread::spawn(move || {
    for i in 1..10 {
      println!("hi number {i} from the spawned thread!");
      thread::sleep(Duration::from_millis(1));

      tx.send("jdjjddjdj".to_string()).unwrap();
    }
  });

  println!("finished? {}", handle.is_finished());
  // let xmm = tx.send("jj".to_string());

  // let b = rx.recv().unwrap();

  for j in rx {
    println!("j {j}")
  }

  for i in 1..15 {
    println!("finished? {} ", handle.is_finished(),);

    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();

  // println!("{x}");
}
