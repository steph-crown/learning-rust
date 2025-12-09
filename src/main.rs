fn main() {
    let value = 40.0;
    let celsius_value = fah_to_cel(value);
    let fah_value = cel_to_fah(celsius_value);

    println!("{value}F to Celsius is {}C", celsius_value);
    println!("{celsius_value}C to Fahrenheit is {fah_value}F");
}

fn fah_to_cel(val: f32) -> f32 {
    (val - 32.0) * (5.0 / 9.0)
}

fn cel_to_fah(val: f32) -> f32 {
    ((9.0 / 5.0) * val) + 32.0
}
