//https://atcoder.jp/contests/abc087/tasks/abc087_b

use std::io;

fn read() -> i32 {
    let mut x = String::new();
    io::stdin().read_line(&mut x).unwrap();
    let x: i32 = x.trim().parse().unwrap();
    x
}

fn main(){
    let a = read();
    let b = read();
    let c = read();
    let x = read();
    let mut result = 0;
    let mut sum = 0;

    for i in 0..a+1{
        for j in 0..b+1{
            for k in 0..c+1{
                sum = 500*i + 100*j + 50*k;
                if sum==x{
                    result += 1;
                }
                if sum>x{
                    break;
                }
            }
            
        }
    }
    println!("{}", result); 
}