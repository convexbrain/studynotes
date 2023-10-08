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

    let h: u32 = token.next().unwrap().parse().unwrap();
    let w: u32 = token.next().unwrap().parse().unwrap();

    dprintln!("{} {}", h, w);

    if h == 1 || w == 1 {
        println!("{}", h * w);
    }
    else {
        let n = ((h + 1) / 2) * ((w + 1) / 2);

        println!("{}", n);
    }
}
