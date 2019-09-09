//https://atcoder.jp/contests/abc085/tasks/abc085_b

use std::io;
use std::collections::HashMap;

fn read() -> i32{
    let mut n = String::new();
     io::stdin().read_line(&mut n).unwrap();
     let n: i32 = n.trim().parse().unwrap();
     n
}
 
 fn main(){
    let n = read();
    let mut d = HashMap::new();

    for _i in 0..n{
        let key = read();
        match d.get(&key){
            Some(_e) => continue,
            None => d.insert(key, 0),
        };
    }
    println!("{}", d.len());
 }