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
    let s = token.next().unwrap();
    debug!(n, s);

    let sb = s.as_bytes();

    let mut si = 0;
    let mut ei = 0;
    let mut c = sb[0];

    let mut alpha = vec![0; 26];

    loop {
        if ei < s.len() {
            let tc = sb[ei];

            if tc == c {
                let ai = (c - b'a') as usize;
                alpha[ai] = alpha[ai].max(ei - si + 1);

                ei += 1;
            }
            else {
                c = tc;

                let ai = (c - b'a') as usize;
                alpha[ai] = alpha[ai].max(1);

                si = ei;
                ei += 1;
            }
        }
        else {
            break;
        }
    }

    let ans = alpha.iter().fold(0, |acc, x| acc + x);
    
    println!("{ans}");
}
