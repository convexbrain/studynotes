use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            eprint!("@{}:", line!());
            eprintln!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut s = 1_u64;
    for _ in 0..n {
        let sum_a = (0..6).map(|_| token.next().unwrap().parse().unwrap())
                       .fold(0, |acc, x: u64| acc + x);
        s = (s * sum_a) % 1000000007;
    }

    println!("{}", s);
}
