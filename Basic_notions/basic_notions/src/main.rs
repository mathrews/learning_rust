use std::{f64::consts, usize};

fn main() {
    println!("Hello, world!");

    // let1
    let answer = 42;
    println!("Hello {}\n", answer);

    // let2
    let answer = 42;
    assert_eq!(42, answer);

    // for1
    for i in 0..5 {
        println!("Hello {}", i);
    }
    println!("");

    // for2
    for i in 0..5 {
        if i % 2 == 0 {
            println!("even: {}", i);
        } else {
            println!("odd: {}", i);
        }
    }
    println!("");

    // for3
    for i in 0..5 {
        let even_odd = if i % 2 == 0 { "even" } else { "odd" };
        println!("{}: {}", even_odd, i);
    }
    println!("");

    // add1 (wrong because sum isn't mutable)
    // let sum = 0;
    // for i in 0..5 {
    //      sum += i;
    // }
    // println!("sum is {}", sum);

    // add2
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum is {}", sum);

    // add3
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {:.1}", sum);

    // fn1
    fn sqr(x: f64) -> f64 {
        x * x
    }
    println!("{}", sqr(2.0));

    // fn2
    let x = 5.0;
    fn abs(x: f64) -> f64 {
        if x > 0.0 {
            x
        } else {
            -x
        }
    }
    println!("abs of {x}: {}", abs(x));

    // fn3
    fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
        if x < x1 {
            x1
        } else if x > x2 {
            x2
        } else {
            x
        }
    }
    println!("{}", clamp(x, 3.0, 4.0));

    // fn4
    fn factorial(n: f64) -> f64 {
        if n == 0.0 {
            1.0
        } else {
            n * factorial(n - 1.0)
        }
    }
    println!("{}", factorial(5.0));

    // fn5
    fn by_ref(x: &i32) -> i32 {
        *x + 1
    }
    let i = 10;
    println!("{} {}", by_ref(&i), by_ref(&41));

    // fn6 (if you want modify the args)
    fn by_ref_mut(x: &mut i32) {
        *x += 1;
    }
    let mut i = 10;
    by_ref_mut(&mut i);
    println!("i is {}", i);

    // learning where to find the ropes
    // ex1: finding math example
    let pi: f64 = 3.1416;
    let x = pi / 2.0;
    let cosine = x.cos();
    println!("{cosine}");

    // ex2:
    let x = 2.0 * std::f64::consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    println!("{abs_difference}");
    assert!(abs_difference < 1e-10);

    // ex3:
    let x = 2.0 * consts::PI;
    let abs_difference = (x.cos() - 1.0).abs();
    println!("{abs_difference}");
    assert!(abs_difference < 1e-10);
    println!();

    // array1
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);

    for (index, item) in arr.iter().enumerate() {
        println!("[{}] = {}", index, item);
    }
    println!("length {}", arr.len());
}
