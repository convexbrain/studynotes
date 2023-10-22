use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

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

    let n: u32 = token.next().unwrap().parse().unwrap();
    let l: u32 = token.next().unwrap().parse().unwrap();
    let k: u32 = token.next().unwrap().parse().unwrap();

    debug!(n, l, k);

    let a: Vec<u32> = (0..n).map(|_| token.next().unwrap().parse().unwrap()).collect();

    debug!(a);

    let mut score_lt = l;
    let mut score_ge = 1;

    loop {
        debug!(score_ge, score_lt);

        let score_check = (score_ge + score_lt) / 2;
        debug!(score_check);
        if score_check == score_ge {
            break;
        }

        let mut p_len = 0;
        let mut pos = 0_usize;
        let mut cut = 0;
        while pos < n as usize && cut < k {
            if a[pos] - p_len >= score_check {
                debug!(pos, cut);
                p_len = a[pos];
                cut += 1;
            }
            pos += 1;
        }
        
        if cut == k && l - p_len >= score_check {
            score_ge = score_check;
        }
        else {
            score_lt = score_check;
        }
    }
    
    println!("{}", score_ge);
}
