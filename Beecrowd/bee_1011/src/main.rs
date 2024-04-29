use std::io;

const PI_VALUE: f64 = 3.14159;

fn main() {
    let mut input_radius: String = String::new();
    io::stdin().read_line(&mut input_radius).unwrap();

    let input_conv = input_radius.trim().parse::<i32>().unwrap_or_default();
    let input_f64 = f64::from(input_conv);

    println!(
        "VOLUME = {:.3}",
        ((4.0 / 3.0) * PI_VALUE * input_f64.powf(3.0))
    );
}
