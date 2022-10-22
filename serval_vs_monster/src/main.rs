use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let xs: Vec<i32> = input()
        .split_whitespace()
        .into_iter()
        .map(|e| e.parse().unwrap())
        .collect();
    let q = xs[0] / xs[1];
    let r = xs[0] % xs[1];
    match r {
        0 => println!("{}", q),
        _ => println!("{}", q + 1),
    }
}
