use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    str, rc::*, cell::*,
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

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    let mut comb = vec![0; n];
    let mut col = vec![0; n];

    for _ in 0..n {
        let s = token.next().unwrap(); // &str
        debug!(s);

        let mut n_o = 0;
        for (j, val) in s.bytes().enumerate() {
            if val == b'o' {
                n_o += 1;
                col[j] += 1;
            }
        }

        let c2 = if n_o < 2 {0} else {n_o - 1};
        debug!(c2);
        for (j, val) in s.bytes().enumerate() {
            if val == b'o' {
                comb[j] += c2;
            }
        }
        debug!(comb);
    }
    debug!(col);
    debug!(comb);

    let mut ans = 0_usize;
    for i in 0..n {
        ans += comb[i] * (col[i].max(1) - 1);
    }

    println!("{ans}");
}
