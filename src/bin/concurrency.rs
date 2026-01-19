use std::thread;
use std::time::Duration;

fn main() {
  let x = String::from("kkkeek");
  let handle = thread::spawn(move || {
    for i in 1..10 {
      println!("hi number {i} from the spawned thread! {x}");
      thread::sleep(Duration::from_millis(1));
    }
  });

  println!("finished? {}", handle.is_finished());

  for i in 1..15 {
    println!("finished? {}", handle.is_finished());

    println!("hi number {i} from the main thread!");
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();

  // println!("{x}");
}
