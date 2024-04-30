use std::{f64, io};

struct Dimensions {
    a: f64,
    b: f64,
    c: f64,
}

impl Dimensions {
    fn new(a: f64, b: f64, c: f64) -> Dimensions {
        Dimensions { a: a, b: b, c: c }
    }
}

const PI_VALUE: f64 = 3.14159;

fn main() {
    let mut inputs: Vec<String> = Vec::new();
    for _ in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        inputs.push(input);
    }

    let mut inputs_conv: Vec<f64> = Vec::new();
    for input in inputs {
        let input_conv = input.trim().parse::<f64>().unwrap_or_default();
        inputs_conv.push(input_conv);
    }

    let dimensoes = Dimensions::new(inputs_conv[0], inputs_conv[1], inputs_conv[2]);

    println!("TRIANGULO: {:.3}", ((dimensoes.a * dimensoes.c) / 2.0));
    println!("CIRCULO: {:.3}", dimensoes.c.powi(2) * PI_VALUE);
    println!("TRAPEZIO: {:.3}", (1.0/2.0 * (dimensoes.a + dimensoes.b) * dimensoes.c));
    println!("QUADRADO: {:.3}", dimensoes.b.powi(2));
    println!("RETANGULO: {:.3}", dimensoes.a * dimensoes.b);
}
