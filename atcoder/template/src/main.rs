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

    let a = token.next().unwrap(); // &str
    let b: String = token.next().unwrap().to_string();
    let c: Vec<char> = token.next().unwrap().chars().collect();
    let d = token.next().unwrap().as_bytes(); // &[u8]
    let n: usize = token.next().unwrap().parse().unwrap();

    dprintln!("{} {} {:?} {:?} {}", a, b, c, d, n);
    println!("This is a template.");
}
