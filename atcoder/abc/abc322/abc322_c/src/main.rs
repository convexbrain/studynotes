use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: usize = spl.next().unwrap().parse().unwrap();
    let m: usize = spl.next().unwrap().parse().unwrap();

    eprintln!("{} {}", n, m);

    let mut i = 1;

    for _ in 0.. m {
        let a: u32 = spl.next().unwrap().parse().unwrap();

        for ii in i..= a {
            let d = a - ii;
            println!("{}", d);
        }

        i = a + 1;
    }
}
