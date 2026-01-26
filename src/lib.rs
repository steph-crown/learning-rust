use std::{
  sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
  },
  thread::{self, JoinHandle},
};

pub mod http;
pub mod tcp_server;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  /// Create a new ThreadPool.
  ///
  /// The size is the number of threads in the pool.
  ///
  /// # Panics
  ///
  /// The `new` function will panic if the size is zero.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    Self { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);

    self.sender.send(job).unwrap();
  }
  // pub fn execute() {}
}

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || {
      println!("sissisisi");

      loop {
        let job = receiver.lock().unwrap().recv().unwrap();

        println!("Worker {id} got a job; executing.");

        job();
      }
    });

    Worker { id, thread }
  }
}
