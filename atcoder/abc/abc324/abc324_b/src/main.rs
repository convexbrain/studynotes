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

    let mut n: u64 = token.next().unwrap().parse().unwrap();

    while (n > 1) && (n & 1) == 0 {
        n >>= 1;
    }

    while (n > 1) && (n % 3) == 0 {
        n /= 3;
    }

    if n == 1 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
