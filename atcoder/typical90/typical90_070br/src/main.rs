use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*,
    rc::*, cell::*, ops::Bound::*,
};

#[cfg(not(debug_assertions))]
macro_rules! debug {
    ( $($x:tt)* ) => {};
}

#[cfg(debug_assertions)]
macro_rules! debug {
    () => {
        eprintln!("[@{}]", line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            ref tmp => {
                eprintln!("[@{}] {} = {:?}",
                    line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(debug!($val)),+,)
    };
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        x[i] = token.next().unwrap().parse().unwrap();
        y[i] = token.next().unwrap().parse().unwrap();
    }

    debug!(n, x, y);
    
    x.sort();
    y.sort();

    let mx: i32 = x[n / 2];
    let my: i32 = y[n / 2];

    let s = x.iter().zip(y.iter()).fold(0_u64, |acc, xy| {
        acc + mx.abs_diff(*xy.0) as u64 + my.abs_diff(*xy.1) as u64
    });

    println!("{s}");
}
