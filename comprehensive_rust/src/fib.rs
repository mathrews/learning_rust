pub fn fib(x: i32) -> i32 {
    if x < 2 {
        x
    } else {
        fib(x - 1) + fib(x - 2)
    }
}
