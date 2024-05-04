use std::io::{self};

trait Distance {
    fn distance(&self) -> f64;
}

struct Points {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Distance for Points {
    fn distance(&self) -> f64 {
        f64::sqrt((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2))
    }
}

fn main() {
    let inputs = input_line();
    let points = Points { x1: inputs[0], y1: inputs[1], x2: inputs[2], y2: inputs[3] };
    println!("{:.4}", points.distance());
}

fn input_line() -> Vec<f64> {
    let mut inputs = Vec::new();
    
    for _ in 0..4 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        inputs.push(input);
    }

    let mut inputs_conv = Vec::new();
    for input in inputs {
        let input_conv = input.trim().parse::<f64>().unwrap_or_default();
        inputs_conv.push(input_conv);
    }

    return inputs_conv;
}
