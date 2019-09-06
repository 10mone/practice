use std::io;

fn read_as_vec() -> Vec<i32> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace()
        .map(|e| e.parse().ok().unwrap()).collect()
}

fn main(){
    /*
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:i32 = n.trim().parse().unwrap();
    */
    //let mut input = read_as_vec();
    

    //標準入力がないと成功する。
    //
    let mut input = [8, 12, 40];
    let mut cnt = 0;
    let n = 3;
    
    loop{
        let mut exist_odd = false;
        for i in &input{
            if i%2 != 0{
                exist_odd = true
            }
        }

        if exist_odd {
            break;
        }
    
        for i in 0..n{
            input[i] = input[i]/2;
        }
        cnt += 1;
        println!("{:?}",input);
    }
    println!("{}", cnt);
}