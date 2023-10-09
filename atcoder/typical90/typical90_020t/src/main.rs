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

    let a: u64 = token.next().unwrap().parse().unwrap();
    let b: u32 = token.next().unwrap().parse().unwrap();
    let c: u64 = token.next().unwrap().parse().unwrap();

    dprintln!("{} {} {}", a, b, c);

    if a < c.pow(b) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
