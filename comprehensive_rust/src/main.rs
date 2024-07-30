use std::time::Duration;

use comprehensive_rust::{
    collatz::{collatz_length, collatz_recursive},
    fib::fib,
    nested_arrays::transpose,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:-^80}", "Day 1 morning");
    println!("Hello, World.");

    // variables
    let mut x: i32 = 10;
    println!("x = {x}");
    x = 20;
    println!("x = {x}");

    // arithmetic
    println!("result: {}", interproduct(120, 100, 248));

    // type inference
    let x = 10;
    let y = 20;
    takes_u32(x);
    takes_i8(y);
    takes_u32(y.try_into()?); // if u put just "y", will be an error because
                              // the rust has made the inference in the call above. But if you
                              // convert the variable with methods .try_into().unwrap(), the
                              // program will run fine.

    // fibonacci exercise
    let x = 6;
    println!("fib({x}) = {}", fib(x));

    // Control Flow Basics
    // if expressions
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
    // as expression
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);

    // loops:
    // while
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");

    // for
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }

    // loop
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }

    // break and continue
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        println!("{}", i);
    }
    // labels:
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");

    // Blocks and Scopes
    // Blocks
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");

    // Scopes and Shadowing:
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }
    println!("after: {a}");

    // Functions:
    println!("gcd: {}", gcd(143, 52));

    // Macros
    fn factorial(n: u32) -> u32 {
        let mut product = 1;
        for i in 1..=n {
            product *= dbg!(i);
        }
        product
    }

    fn fizzbuzz(n: u32) -> u32 {
        todo!()
    }
    let n = 4;
    println!("{n}! = {}", factorial(n));
    println!();

    // collatz
    println!("{}", collatz_length(5));
    println!();

    // Tuples and Arrays:
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:?}");

    let t: (i8, bool) = (7, true);
    println!("t.0 = {}", t.0);
    println!("t.1 = {}", t.1);

    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }

    print_tuple((1, 2));
    print_tuple_patterned((1, 2));

    transpose([
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ]);
    println!();

    // Matching Values
    let input = 'x';
    match input {
        'q' => println!("Quitting"),
        'a' | 's' | 'w' | 'd' => println!("Moving around"),
        '0'..='9' => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _ => println!("something else"),
    }
    println!();

    // Structs
    let foo = Foo {
        x: (1, 2),
        y: 2,
        z: 40,
    };
    match foo {
        Foo {
            x: (a, 2),
            y,
            z: 20,
        } => println!("x.0 = {a}, b = 2, y = {y}, z = 20"),
        Foo { y: 2, x: i, z: 40 } => println!("y = 2, x = {i:?}, z = 40"),
        Foo { y, .. } => println!("y = {y}, other fields were ignored"),
    }
    println!();

    // enums
    let n = 99;
    match divide_in_two(n) {
        Resultado::Okei(half) => println!("{n} divided in two is {half}"),
        Resultado::Erro(msg) => println!("sorry, an error happened: {msg}"),
    }
    println!();

    // for n in 1..=100 {
    //     let result: Resultado = divide_in_two(n);
    //     match &result {
    //         Resultado::Okei(half) => println!("{n} divided in two is {half}"),
    //         Resultado::Erro(msg) => println!("sorry, an error happened: {msg}"),
    //     }
    // }
    // println!();

    // Let Control Flow:
    // if let
    fn sleep_for(secs: f32) {
        if let Ok(dur) = Duration::try_from_secs_f32(secs) {
            std::thread::sleep(dur);
            println!("slept for {:?}", dur);
        }
    }
    sleep_for(-10.0);
    sleep_for(0.8);

    // let else
    fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
        let Some(s) = maybe_string else {
            return Err(String::from("got None"));
        };

        let Some(first_byte_char) = s.chars().next() else {
            return Err(String::from("got empty string"));
        };

        let Some(digit) = first_byte_char.to_digit(16) else {
            return Err(String::from("not a hex digit"));
        };

        return Ok(digit);
    }
    println!("result: {:?}", hex_or_die_trying(Some(String::from("bar"))));

    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        println!("character: {c}");
    }
    println!();

    Ok(())
}

struct Foo {
    x: (u32, u32),
    y: u32,
    z: u32,
}

enum Resultado {
    Okei(i32),
    Erro(String),
}

fn divide_in_two(n: i32) -> Resultado {
    if n % 2 == 0 {
        Resultado::Okei(n / 2)
    } else {
        Resultado::Erro(format!("Cannot divide {n} into two equal parts"))
    }
}

fn print_tuple(tuple: (i32, i32)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}

fn print_tuple_patterned(tuple: (i32, i32)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}

fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else if b <= 20 {
        return b;
    } else {
        a
    }
}

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}

fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}
