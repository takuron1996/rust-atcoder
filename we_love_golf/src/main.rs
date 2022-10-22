use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let k: i32 = input().parse().unwrap();
    let xs: Vec<i32> = input()
        .split_whitespace()
        .into_iter()
        .map(|e| e.parse().unwrap())
        .collect();
    if xs[1] / k * k >= xs[0] {
        println!("OK")
    } else {
        println!("NG")
    }
}
