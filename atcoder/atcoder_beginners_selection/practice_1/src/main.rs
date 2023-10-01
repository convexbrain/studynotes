use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf).unwrap();

    let mut spl = buf.split_whitespace();

    let a: u32 = spl.next().unwrap().parse().unwrap();
    let b: u32 = spl.next().unwrap().parse().unwrap();
    let c: u32 = spl.next().unwrap().parse().unwrap();
    let s = spl.next().unwrap();

    let sum = a + b + c;

    println!("{} {}", sum, s);
}
