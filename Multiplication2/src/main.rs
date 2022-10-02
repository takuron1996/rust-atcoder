use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Faild to read line");
    buf.trim().to_string()
}

fn main() {
    let _ = input();
    let xs: Vec<i64> = input()
        .split_whitespace()
        .into_iter()
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
    if xs.iter().any(|e| *e == 0) {
        println!("0");
        return;
    }
    let mut result: i64 = 1;
    for x in &xs {
        if result > 10_i64.pow(18) / x {
            result = -1;
            break;
        }
        result *= x;
    }
    println!("{}", result);
}
