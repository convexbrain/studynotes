use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;

macro_rules! dprintln {
    ( $($x:tt)* ) =>
    {
        #[cfg(debug_assertions)]
        {
            print!("[{}]", line!());
            println!($($x)*);
        }
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut spl = buf.split_whitespace();

    let a = spl.next().unwrap();
    let b = spl.next().unwrap();
    let c = spl.next().unwrap();
    let d: u32 = spl.next().unwrap().parse().unwrap();

    dprintln!("{} {} {} {}", a, b, c, d);
    println!("{} {} {} {}", a, b, c, d);
}
