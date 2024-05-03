use std::io::{self};

trait Distance {
    fn distance(&self) -> f64;
}

struct Points {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Distance for Points {
    fn distance(&self) -> f64 {
        f64::sqrt((self.x2 - self.x1).powi(2) + (self.y2 - self.y1).powi(2))
    }
}

fn main() {
    let (x1, y1, x2, y2) = input_line();
    let points = Points { x1: x1, y1: y1, x2: x2, y2: y2 };
    println!("{:.4}", points.distance());
}

fn input_line() -> (f64, f64, f64, f64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let [a, b, c, d]: [String; 4] = <[String; 4]>::try_from(
        line.split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>(),
    )
    .unwrap_or_default();

    let (x1, y1, x2, y2): (f64, f64, f64, f64) = (
        a.parse::<f64>().unwrap_or_default(),
        b.parse::<f64>().unwrap_or_default(),
        c.parse::<f64>().unwrap_or_default(),
        d.parse::<f64>().unwrap_or_default(),
    );

    return (x1, y1, x2, y2);
}
