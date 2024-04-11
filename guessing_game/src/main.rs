use rand::Rng;
use std::cmp::Ordering as Ordenacao;
use std::{i32, io};

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new(); // on here, are being declared a mutable variable

        io::stdin()
            .read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: i32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordenacao::Less => println!("Too small!\n"),
            Ordenacao::Greater => println!("Too big!\n"),
            Ordenacao::Equal => {
                println!("You win\n");
                break;
            }
        }
    }

    // example of use {}:
    let x: i32 = 5;
    let y: i32 = 10;

    println!("x = {x} and y + 2 = {}", y + 2);
}
