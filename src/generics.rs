use std::fmt::Debug;

struct Point<T> {
  x: T,
  y: T,
}

impl Point<i32> {
  fn x(&self) -> &i32 {
    &self.x
  }
}

pub fn run() {
  let x = ['1', '2'];
  println!("{}", largest(&x));

  // let y = Point { x: 4, y: 5 };
  // println!("y.x(): {}", y.x());
}

fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn largest<T>(list: &[T]) -> &T
where
  T: PartialOrd + Debug,
{
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  println!("list {:#?}", list);

  largest
}
