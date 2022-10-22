use std::io;

fn input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Faild to read line");
    buf.trim().to_string()
}

fn main() {
    // atcoderのRustのバージョンではreduceが実行できないのでfoldを使用
    let x: i32 = input()
        .split_whitespace()
        .into_iter()
        .map(|e| e.parse().unwrap())
        .reduce(|a, b| a * b)
        .unwrap();
    // let x: i32 = input()
    //     .split_whitespace()
    //     .into_iter()
    //     .map(|e| e.parse().unwrap())
    //     .fold(1, |a, b: i32| a * b);
    match x % 2 {
        0 => println!("Even"),
        _ => println!("Odd"),
    };
}
