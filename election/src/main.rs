use std::collections::HashMap;
use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let n: u32 = input().parse().unwrap();
    let mut xs = HashMap::new();

    for _ in 0..n {
        let key = input();
        *xs.entry(key).or_insert(0) += 1;
    }
    println!("{}", xs.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0);
}
