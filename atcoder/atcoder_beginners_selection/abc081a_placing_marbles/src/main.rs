use std::io;
use std::io::prelude::*;

fn main() {
    let mut buf = String::new();

    io::stdin().read_to_string(&mut buf).unwrap();

    let mut spl = buf.split_whitespace();

    let s = spl.next().unwrap();

    let mut sum = 0;
    for c in s.chars() {
        if c == '1' {
            sum += 1
        }
    }
    println!("{}", sum);
}
