use comprehensive_rust::fib::fib;

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

    Ok(())
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
