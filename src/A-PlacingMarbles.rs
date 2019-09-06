//https://atcoder.jp/contests/abc081/tasks/abc081_a
use std::io;

fn read_as_vec() -> Vec<char>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<char> = input.chars().collect(); 
    input
}

fn main(){
    let mut input: Vec<char> = read_as_vec();
    let len = input.len();
    let mut count = 0;

    for i in 0..len{
        if input[i]=='1'{
            count += 1;
        }
    }
    println!("{}",count);
}

