use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let input_conv = input.trim().parse::<i32>().unwrap_or_default();
    
    println!("{} minutos", input_conv * 2);
}
