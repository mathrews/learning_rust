use std::io::{self};

fn main() {
    let inputs = input_line();
    let maior_ab = (inputs[0] + inputs[1] + i32::abs(inputs[0] - inputs[1])) / 2;

    println!("{} eh o maior", (maior_ab + inputs[2] + i32::abs(maior_ab - inputs[2])) / 2);
    
}

fn input_line() -> Vec<i32> {
    let mut inputs = Vec::new();
    
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        inputs.push(input);
    }

    let mut inputs_conv = Vec::new();
    for input in inputs {
        let input_conv = input.trim().parse::<i32>().unwrap_or_default();
        inputs_conv.push(input_conv);
    }

    return inputs_conv;
}
