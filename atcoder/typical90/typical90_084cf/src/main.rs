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

    let n: usize = token.next().unwrap().parse().unwrap();
    let s = token.next().unwrap().as_bytes(); // &[u8]

    debug!(n, s);

    let mut chg = vec![0; n];

    let mut i = 0;
    let mut maru = s[0] == b'o';
    for j in 1..n {
        if s[j] == (if maru {b'x'} else {b'o'}) {
            for k in i..j {
                chg[k] = j;
            }
            i = j;
            maru = !maru;
        }
    }

    debug!(chg);

    let mut cnt = 0;

    for &val in chg.iter() {
        if val == 0 {
            break;
        }
        else {
            let c = n - val;
            debug!(c);
            cnt += c;
        }
    }
    
    println!("{}", cnt);
}
