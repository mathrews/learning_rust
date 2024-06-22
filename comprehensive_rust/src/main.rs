fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
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

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
