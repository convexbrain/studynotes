use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: usize = spl.next().unwrap().parse().unwrap();
    let m: usize = spl.next().unwrap().parse().unwrap();
    let mut a = vec![0; m];

    for j in 0.. m {
        let v: usize = spl.next().unwrap().parse().unwrap();
        a[j] = v; 
    }

    eprintln!("{} {} {:?}", n, m, a);

    for i in 1..= n {
        let mut d = 0;

        for j in 0.. m {
            if i <= a[j] {
                d = a[j] - i;
                break;
            }
        }

        println!("{}", d);
    }
}
