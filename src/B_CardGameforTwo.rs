//https://atcoder.jp/contests/abc088/tasks/abc088_b

use std::io;
use std::cmp::Reverse;

fn read_vec() -> Vec<i32>{
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace()
                .map(|e| e.parse().ok().unwrap()).collect()
}

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut cards = read_vec();
    let mut alice = 0;
    let mut bob = 0;

    cards.sort_by_key(|&x| Reverse(x));

    let mut i = 0;
    for card in &cards{
        if i%2==0 {
            alice += cards[i];
            i += 1;
        }else{
            bob += cards[i];
            i += 1;
        }
    }
    println!("{}",alice - bob);
}

