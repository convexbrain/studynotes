use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: u32 = spl.next().unwrap().parse().unwrap();
    let a: u32 = spl.next().unwrap().parse().unwrap();
    let b: u32 = spl.next().unwrap().parse().unwrap();

    eprintln!("{} {} {}", n, a, b);

    let mut cnt = 0;

    for i in 1..= n {
        let d0 = i % 10;
        let d1 = (i / 10) % 10;
        let d2 = (i / 100) % 10;
        let d3 = (i / 1000) % 10;
        let d4 = (i / 10000) % 10;
        let ds = d0 + d1 + d2 + d3 + d4;
        eprintln!("{} {}", i, ds);
        if a <= ds && ds <= b {
            cnt += i;
        }
    }

    println!("{}", cnt);
}
