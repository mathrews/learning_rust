use core::str;
use std::{f64::consts, fmt::format, string};

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

    // slice1
    fn sum_fn(values: &[i32]) -> i32 {
        let mut res = 0;
        for i in 0..values.len() {
            res += values[i];
        }
        res
    }
    let arr = [20, 40, 60];
    let res = sum_fn(&arr);
    println!("sum {}", res);

    // array3
    let ints = [1, 2, 3];
    let floats = [1.0, 2.0, 3.0];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
    println!();

    // slice2
    let ints = [1, 2, 3, 4, 5];
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];
    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
    println!();

    // slice3
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
    println!();

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), first.is_none());
    println!("first value {}", *first.unwrap());
    println!();

    let maybe_last = slice.get(4);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap() // deference because the type of inside the some is &i32;
    } else {
        -1
    };
    println!("{last}");

    let _last = *slice.get(5).unwrap_or(&-1);

    // Vec1
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0]; // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
    println!();

    // Vec2
    fn dump(arr: &[i32]) {
        println!("arr is {:?}", arr);
    }

    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
    println!();

    // iter1
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // iter2
    let arr = [10, 20, 30];
    for i in arr {
        println!("{}", i);
    }
    println!();

    // iter3
    let arr = [10, 20, 30];
    for i in arr.iter() {
        println!("{}", i);
    }

    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }
    println!();

    // sum1
    let sum: i32 = (0..5).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
    println!();

    // slice4
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
    println!();

    for s in slice.chunks(2) {
        println!("chunck {:?}", s);
    }
    println!();

    // vec3
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
    println!();

    // vec4
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);

    // string1
    fn dump2(s: &str) {
        println!("str '{}'", s);
    }
    let text = "hello dolly"; // uma &str
    let s = text.to_string(); // uma string alocada

    dump2(text);
    dump2(&s);
    println!();

    // string5
    let mut s = String::new();
    // initially empty!
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
                   // remove the last char;
    s.pop();

    assert_eq!(s, "Hello World");
    println!();

    // string6
    fn array_to_str(arr: &[i32]) -> String {
        let mut res = '['.to_string();
        for v in arr {
            res += &v.to_string();
            res.push(',');
        }
        res.pop();
        res.push(']');
        res
    }
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");

    // string2
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
    println!();

    // string3
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}'", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('п');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
    // 'H' 'i' '!' ' ' '¡' 'H' 'o' 'l' 'a' '!' ' ' 'п' 'р' 'и' 'в' 'е' 'т' '!'
    // len 25
    // count 18
    // Russian hi привет!

    // string examples
    let text = "The red fox ant the lazy dog";
    let _words: Vec<&str> = text.split_whitespace().collect();
    // ["the", "red", "fox", "and", "the", "lazy", "dog"]

    let mut words = Vec::new();
    words.extend(text.split_whitespace());

    let _stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
    // theredfoxandthelazydog
}
