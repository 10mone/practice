//https://atcoder.jp/contests/abc086/tasks/abc086_a

use std::io;
use std::str::FromStr;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut temp = input.split_whitespace()
                .map(|e| i32::from_str(e).unwrap());
    let mut a = temp.next().unwrap();
    let mut b = temp.next().unwrap();

    if (a*b)%2==0 {
        println!("Even");
    }else{
        println!("Odd");
    }
}   