use std::io;

fn main() {
    let mut inputs: Vec<String> = Vec::with_capacity(3);

    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        inputs.push(input);
    }

    let mut inputs_converted: Vec<i32> = Vec::with_capacity(2);
    let mut gain_per_hour = 0.0;

    for input in inputs {
        match input.find('.') {
            Some(_) => {
                let input_conv = input.trim().parse::<f64>().unwrap_or_default();
                gain_per_hour = input_conv;
            }
            None => (),
        }
        let input_conv = input.trim().parse::<i32>().unwrap_or_default();
        inputs_converted.push(input_conv);
    }

    let salary = f64::from(inputs_converted[1]) * gain_per_hour;

    println!(
        "NUMBER = {}\nSALARY = U$ {:.2}",
        inputs_converted[0], salary
    );
}
