use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let xs: Vec<u32> = input()
        .split_whitespace()
        .into_iter()
        .map(|e| e.parse().unwrap())
        .collect();
    let mut result: Vec<u32> = Vec::new();
    for x in xs[1]..=xs[0] {
        let y: u32 = x
            .to_string()
            .chars()
            .into_iter()
            .map(|c| c.to_digit(10).unwrap())
            .sum();
        if xs[1] <= y && y <= xs[2] {
            result.push(x);
        }
    }
    println!("{}", result.into_iter().sum::<u32>())
}
