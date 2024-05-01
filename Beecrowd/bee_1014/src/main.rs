use std::io;

fn main() {
    let mut distance = String::new();
    let mut fuel_spent = String::new();

    io::stdin().read_line(&mut distance).unwrap();
    io::stdin().read_line(&mut fuel_spent).unwrap();

    let (distance_conv, fuel_conv): (i32, f64) = (distance.trim().parse::<i32>().unwrap(), fuel_spent.trim().parse::<f64>().unwrap());

    println!("{:.3} km/l", f64::from(distance_conv) / fuel_conv);
}
