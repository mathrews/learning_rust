use std::io;

const SPEND_FUEL: f64 = 12.0;
fn main() {
    let inputs = inputs();
    println!("{:.3}", (f64::from(inputs[0]) * f64::from(inputs[1])) / SPEND_FUEL);
}

fn inputs() -> Vec<i32> {
    let mut inputs = Vec::new();
    for _ in 0..2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_conv = input.trim().parse::<i32>().unwrap_or_default();
        inputs.push(input_conv);
    }
    inputs
}

