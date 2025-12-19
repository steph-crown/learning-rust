use core::f64;
use std::{
  cmp::{max, min},
  io,
};

#[derive(Debug)]
struct User {
  name: String,
  email: String,
  gender: String,
  active: bool,
}

#[derive(Debug)]
struct Coords(i32, i32, i32);

#[derive(Debug)]
struct Color(i32, i32, String);

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

fn main() {
  // let value = 40.0;
  // let celsius_value = fah_to_cel(value);
  // let fah_value = cel_to_fah(celsius_value);

  // println!("{value}F to Celsius is {}C", celsius_value);
  // println!("{celsius_value}C to Fahrenheit is {fah_value}F");

  // // fibonacci
  // let num = 4;
  // let fib_val = get_fib_loop(num);
  // let fib_val_rec = get_fib_recur(num);
  // println!("zeroth number in Fib sequence is loop {fib_val}");
  // println!("zeroth number in Fib sequence is rec {fib_val_rec}");

  // twelve_days_christmas(12);

  // start_calculator();

  // let mut user = User {
  //   name: String::from("Steph Crown"),
  //   email: String::from("a@ma"),
  //   gender: String::from("Male"),
  //   active: false,
  // };

  // practice_struct();
  // practice_tuple_struct();

  let area = rectangle_area(3, 4);
  print!("{area}");

  let rectangle = Rectangle {
    width: 3,
    height: 9,
  };

  let rectangle2 = Rectangle {
    width: 9,
    height: 2,
  };

  let area_2 = rectangle_area_struct(&rectangle);
  let can_rect_contain = rectangle.can_hold(&rectangle2);
  println!("as struct {area_2} {:#?}", rectangle.area());
  println!("can rectangle contain rectangle2? {can_rect_contain}");

  let the_square = Rectangle::square(8);
  println!("{:#?}", the_square);
}

fn rectangle_area(width: u32, height: u32) -> u32 {
  width * height
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, rect: &Rectangle) -> bool {
    let self_max = max(self.width, self.height);
    let self_min = min(self.width, self.height);

    let rect_max = max(rect.width, rect.height);
    let rect_min = min(rect.width, rect.height);

    self_max > rect_max && self_min > rect_min
  }

  fn square(side: u32) -> Self {
    Self {
      width: 34.max(side),
      height: side,
    }
  }
}

fn rectangle_area_struct(Rectangle { width, height }: &Rectangle) -> u32 {
  // let Rectangle { width, height } = rect;
  width * height
}

fn practice_tuple_struct() {
  let mut a = (2, 3, 4);
  a = (4, 5, 6);

  let b = Color(3, 4, String::from("5"));
  let Color(x, y, z) = b;

  println!("{:>?}, {x} {y} {z}", a);
}

// fn practice_struct() {
//   let name = String::from("Steph");
//   let email = String::from("love@mail.com");

//   let mut user = build_user(email, name);

//   user.name = String::from("Cum");

//   println!("{:?} name: {}", user, user.name);

//   let user2 = User {
//     email: String::from("heartbreak"),
//     name: String::from("value"),
//     gender: String::from("value"),
//     ..user
//   };

//   let User {
//     email: email,
//     name: name,
//     gender: gender,
//     active: active,
//   } = user2;

//   println!("{:?} user", user);
// }

fn build_user(email: String, name: String) -> User {
  User {
    email,
    name,
    gender: String::from("Male"),
    active: false,
  }
}

fn fah_to_cel(val: f32) -> f32 {
  (val - 32.0) * (5.0 / 9.0)
}

fn cel_to_fah(val: f32) -> f32 {
  ((9.0 / 5.0) * val) + 32.0
}

// using loop
fn get_fib_loop(n: u128) -> u128 {
  if n < 3 {
    return n - 1;
  }

  let mut a = 0;
  let mut val = 1;

  for _num in 3..n + 1 {
    (val, a) = (val + a, val);
  }

  return val;
}

// using recursion
fn get_fib_recur(n: u128) -> u128 {
  if n < 3 {
    return n - 1;
  }

  return get_fib_recur(n - 1) + get_fib_recur(n - 2);
}

fn twelve_days_christmas(n: usize) {
  if n > 12 || n < 1 {
    println!("Error!. Must be between 1 and 12");
    return;
  }

  const GIFTS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
  ];

  const ORD: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth", "tenth",
    "eleventh", "twelvth",
  ];

  let ord_string = ORD[n - 1];

  println!("On the {ord_string} day of Christmas, my true love sent to me");

  for num in (0..n).rev() {
    let gift = GIFTS[num];
    println!("{gift}")
  }
}

fn start_calculator() {
  loop {
    println!("New calculator Session. ");

    println!("Enter the left operand");
    let input = accept_input();

    let left: f64 = match input.trim().parse() {
      Result::Ok(num) => num,
      Result::Err(_) => {
        println!("Invalid");
        return;
      }
    };

    println!("Enter the right operand");
    let input = accept_input();

    let right: f64 = match input.trim().parse() {
      Result::Ok(num) => num,
      Result::Err(_) => {
        println!("Invalid");
        return;
      }
    };

    println!("Enter the operator. A for add, S for subtract, M for multiply, and D for divide");
    let input = accept_input();
    let operator = input.trim();

    if operator == "A" {
      let sum = left + right;
      println!("{left} + {right} = {sum}");
    } else if operator == "S" {
      let diff = left - right;
      println!("{left} - {right} = {diff}");
    } else if operator == "M" {
      let prod = left * right;
      println!("{left} * {right} = {prod}");
    } else if operator == "D" {
      let quot = left / right;
      println!("{left} / {right} = {quot}");
    } else {
      println!("Error processing! Invalid operator selected");
    }
  }
}

fn accept_input() -> String {
  let mut input = "".to_string();

  io::stdin()
    .read_line(&mut input)
    .expect("Error reading line");

  return input;
}
