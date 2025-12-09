fn main() {
    let value = 40.0;
    let celsius_value = fah_to_cel(value);
    let fah_value = cel_to_fah(celsius_value);

    println!("{value}F to Celsius is {}C", celsius_value);
    println!("{celsius_value}C to Fahrenheit is {fah_value}F");

    // fibonacci
    let num = 40;
    let fib_val = get_fib_loop(num);
    let fib_val_rec = get_fib_recur(num);
    println!("zeroth number in Fib sequence is loop {fib_val}");
    println!("zeroth number in Fib sequence is rec {fib_val_rec}");

    twelve_days_christmas(12);
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
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelvth",
    ];

    let ord_string = ORD[n - 1];

    println!("On the {ord_string} day of Christmas, my true love sent to me");

    for num in (0..n).rev() {
        let gift = GIFTS[num];
        println!("{gift}")
    }
}
