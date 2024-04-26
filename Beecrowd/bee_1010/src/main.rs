use std::{f64, io};

#[allow(dead_code)]
struct Product {
    code: i32,
    quantity: i32,
    value: f64,
}

impl Product {
    pub fn new(code: i32, quantity: i32, value: f64) -> Product {
        Product {
            code: code,
            quantity: quantity,
            value: value,
        }
    }
}

fn main() {
    let mut products: Vec<Product> = Vec::with_capacity(2);

    for _ in 0..2 {
        let mut fields: Vec<String> = Vec::with_capacity(3);

        for _ in 0..3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            fields.push(input);
        }

        let mut fields_conv: Vec<i32> = Vec::new();
        let mut value_to_pay: f64 = 0.0;

        for input in fields {
            match input.find('.') {
                Some(_) => {
                    value_to_pay = input.trim().parse::<f64>().unwrap_or_default();
                }
                None => fields_conv.push(input.trim().parse::<i32>().unwrap_or_default()),
            }
        }
        println!("{:?}", fields_conv);

        products.push(Product::new(fields_conv[0], fields_conv[1], value_to_pay));
    }

    println!(
        "VALOR A PAGAR: R$ {:.2}",
        (products[0].value * f64::from(products[0].quantity))
            + (products[1].value * f64::from(products[1].quantity)),
    );
}
