use std::io;

fn input() -> String{
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn main() {
    let d: f64 = input().parse().unwrap();
    println!("{}", d / 100.0);
}
