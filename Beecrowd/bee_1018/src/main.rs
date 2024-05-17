use std::io;

fn main() {
    let mut banknotes = (0, 0, 0, 0, 0, 0, 0);

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut input_conv = input.trim().parse::<i32>().unwrap_or_default();

    if input_conv >= 100 {
        banknotes.0 = input_conv / 100 as i32;
        input_conv -= 100 * banknotes.0;
    }
    banknotes.1 = (input_conv / 50) as i32;
    input_conv -= 50 * banknotes.1;
    banknotes.2 = (input_conv / 20) as i32;
    input_conv -= 20 * banknotes.2;
    banknotes.3 = (input_conv / 10) as i32;
    input_conv -= 10 * banknotes.3;
    banknotes.4 = (input_conv / 5) as i32;
    input_conv -= 5 * banknotes.4;
    banknotes.5 = (input_conv / 2) as i32;
    input_conv -= 2 * banknotes.5;
    banknotes.6 = (input_conv / 1) as i32;

    print!("{}", input);
    println!("{} nota(s) de R$ 100,00", banknotes.0);
    println!("{} nota(s) de R$ 50,00", banknotes.1);
    println!("{} nota(s) de R$ 20,00", banknotes.2);
    println!("{} nota(s) de R$ 10,00", banknotes.3);
    println!("{} nota(s) de R$ 5,00", banknotes.4);
    println!("{} nota(s) de R$ 2,00", banknotes.5);
    println!("{} nota(s) de R$ 1,00", banknotes.6);
}
