use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input_conv = input.trim().parse::<i32>().unwrap_or_default();
    let mut time = (0, 0, 0);

    let hours = input_conv / 3600;
    if hours >= 1 {
        input_conv - (hours as i32 * 3600);
        time.0 = hours;
    }
    let minutes = input_conv / 60;
    if minutes >= 1 {
        input_conv - (minutes as i32 * 60);
        time.1 = minutes;
    }
    let seconds_remaining = input_conv -((hours * 3600) + (minutes * 60)) as i32;
    time.2 = seconds_remaining;

    println!("{}:{}:{}", time.0, time.1, time.2);
}
