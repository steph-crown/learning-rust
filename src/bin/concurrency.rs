use std::{
  collections::VecDeque,
  sync::{
    Arc, Mutex,
    mpsc::{self, Receiver, Sender, SyncSender},
  },
  thread::{self, JoinHandle},
};

fn main() {
  // shared_counter();
  producer_consumer_channel();
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

fn producer_consumer() {
  let queue: Arc<Mutex<VecDeque<u32>>> = Arc::new(Mutex::new(VecDeque::new()));

  // producer thread
  let producer_queue: Arc<Mutex<VecDeque<u32>>> = Arc::clone(&queue);

  let producer = thread::spawn(move || {
    for i in 1..101 {
      let mut queue = producer_queue.lock().unwrap();

      queue.push_back(i);
    }
  });

  // consumer thread
  let consumer_queue = Arc::clone(&queue);
  let consumer = thread::spawn(move || {
    let mut consumed_count = 0;

    while consumed_count < 100 {
      let mut q = consumer_queue.lock().unwrap();
      if let Some(val) = q.pop_front() {
        consumed_count += 1;
        println!("{val}");
      }
    }
  });

  producer.join().unwrap();
  consumer.join().unwrap();
}

fn producer_consumer_channel() {
  let (tx, rx): (SyncSender<u32>, Receiver<u32>) = mpsc::sync_channel(10);

  let producer = thread::spawn(move || {
    for i in 1..101 {
      tx.send(i).unwrap();
    }
  });

  for received in rx {
    println!("{received}")
  }

  producer.join().unwrap();
}

// when to use channel vs mutex
