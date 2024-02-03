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

    let s = spl.next().unwrap();

    dprintln!("{}", s);

    for i in 0..s.len() {
        if i & 1 == 0 {
            continue;
        }
        if &s[i..=i] != "0" {
            println!("No");
            return;
        }

    }

    println!("Yes");
}
