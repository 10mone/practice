use std::io;
use std::str::FromStr;

fn main(){

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut it = buf.split_whitespace().map(|n| i32::from_str(n).unwrap());
    let n = it.next().unwrap();
    let a = it.next().unwrap();
    let b = it.next().unwrap();

    let mut sum = 0;

    for i in 1..n+1{
        let t = i / 1000;
        let h = (i-t*1000) / 100;
        let ten = (i-t*1000-h*100) / 10;
        let one = i - (t*1000 + h*100 + ten*10);   
        let temp = t + h + ten + one;
       
        if a<=temp && temp<=b{
            sum += i;
        }
    }
    println!("{}",sum);
}