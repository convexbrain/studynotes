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

    let l: u64 = token.next().unwrap().parse().unwrap();
    let r: u64 = token.next().unwrap().parse().unwrap();

    debug!(l, r);

    let p = 1000000007;

    let mut d = l.ilog10() + 1;
    let mut u = 10_u64.pow(d);

    let mut n = l;
    let mut cnt = 0;

    loop {
        debug!(d, u, n);

        let uu = (u - 1).min(r);
        debug!(uu);
        let w = uu + n;
        let h = uu - n + 1;
        let (w, h) = if w & 1 == 0 {(w / 2, h)} else {(w, h / 2)};
        let nn = (( w % p ) * ( h % p )) % p;
        debug!(nn);
        let c = (nn * d as u64) % p;
        debug!(c);

        cnt = (cnt + c) % p;

        n = u;
        if uu == r {
            break;
        }
        u *= 10;
        d += 1;
    }

    println!("{}", cnt);
}
