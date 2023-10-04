use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let n: u32 = spl.next().unwrap().parse().unwrap();
    let s = spl.next().unwrap();

    eprintln!("{} {}", n, s);

    if let Some(p) = s.find("ABC") {
        println!("{}", p + 1);
    }
    else {
        println!("{}", -1);
    }
}
