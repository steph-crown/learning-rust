#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(val: T) -> Self {
    Self(val)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {name}!");
}

pub fn main() {
  // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

  // println!("{list:?}");

  // let d = MyBox::new(89);
  // println!("d {}", *d);

  // hello(&String::from("jsjss"));

  let x = Rc::new("4".to_string());
  let y = &x;
  let z = Rc::clone(&x);
  let a = Rc::downgrade(&x);
  let b = a.upgrade();
  println!("count {}", Rc::weak_count(&x));
  // let y = Rc::clone(&x);
  // let z = Rc::clone(&x);

  let m = MyBox::new(String::from("Rust"));
  hello(&(*m)[..]);
}
