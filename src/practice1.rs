//https://atcoder.jp/contests/practice/tasks/practice_1

use std::io;
use std::str::FromStr;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let a: i32 = a.trim().parse().unwrap();
    
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut it = buf.split_whitespace().map(|n| i32::from_str(n).unwrap());
    let b: i32= it.next().unwrap();
    let c: i32 = it.next().unwrap();

    let mut sequence = String::new();
    io::stdin().read_line(&mut sequence).unwrap();

    print!("{} {}",a+b+c, sequence)
    
}

