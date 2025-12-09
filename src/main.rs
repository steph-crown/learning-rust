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
