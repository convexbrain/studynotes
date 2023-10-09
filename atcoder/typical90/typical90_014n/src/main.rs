use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) => {
        #[cfg(debug_assertions)]
        {
            print!("@{}:", line!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{}", n);

    let mut a: Vec<u32> = (0..n).map(|_|
        token.next().unwrap().parse().unwrap()
    ).collect();
    a.sort();

    dprintln!("{:?}", a);

    let mut b: Vec<u32> = (0..n).map(|_|
        token.next().unwrap().parse().unwrap()
    ).collect();
    b.sort();

    dprintln!("{:?}", b);

    let e = a.iter().zip(b.iter())
            .fold(0_u64, |acc, (a, b)| acc + a.abs_diff(*b) as u64);

    println!("{}", e);
}
