pub fn collatz_recursive(n: i32) -> u32 {
    if n == 1 {
        return n as u32;
    } else {
        if n % 2 == 0 {
            let n = n / 2;
            collatz_recursive(n)
        } else {
            let n = 3 * n + 1; 
            collatz_recursive(n)
        }
    }  
    
}

pub fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

