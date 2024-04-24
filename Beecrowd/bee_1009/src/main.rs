use std::{f64, io};

fn main() {
    let mut seller_name = String::new();
    let mut salary_and_sales: Vec<String> = Vec::with_capacity(2);

    io::stdin().read_line(&mut seller_name).unwrap();
    for _ in 0..2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        salary_and_sales.push(input);
    }

    let mut salary_and_sales_conv: Vec<f64> = Vec::with_capacity(2);
    for input in salary_and_sales {
        let input_conv: f64 = input.trim().parse().unwrap_or_default();
        salary_and_sales_conv.push(input_conv);
    }

    if salary_and_sales_conv[1] > 0.0 {
        println!(
            "TOTAL = {:.2}",
            salary_and_sales_conv[0] + ((15.0 / 100.0) * salary_and_sales_conv[1])
        );
    } else {
        println!("TOTAL = {:.2}", salary_and_sales_conv[0]);
    }
}
