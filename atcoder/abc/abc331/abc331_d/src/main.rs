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
    let q: usize = token.next().unwrap().parse().unwrap();

    let p: Vec<Vec<u8>> = (0..n).map(|_|
        token.next().unwrap().as_bytes().to_vec()
    ).collect();
    debug!(p);

    let mut total = 0;
    for i in 0..n {
        for j in 0..n {
            if p[i][j] == b'B' {
                total += 1;
            }
        }
    }
    debug!(total);

    for _ in 0..q {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
        let c: usize = token.next().unwrap().parse().unwrap();
        let d: usize = token.next().unwrap().parse().unwrap();
        debug!(a, b, c, d);
        let c = c + 1;
        let d = d + 1;

        let h = c - a;
        let w = d - b;

        let hn = h / n;
        let wn = w / n;

        let mut ans = total * hn * wn;

        let aa = a + hn * n;
        let bb = b + wn * n;

        let mut tmpw = 0;
        for i in aa..c {
            for j in 0..n {
                if p[i % n][j % n] == b'B' {
                    tmpw += 1;
                }
            }
        }
        ans += tmpw * wn;

        let mut tmph = 0;
        for i in 0..n {
            for j in bb..d {
                if p[i % n][j % n] == b'B' {
                    tmph += 1;
                }
            }
        }
        ans += tmph * hn;

        for i in aa..c {
            for j in bb..d {
                if p[i % n][j % n] == b'B' {
                    ans += 1;
                }
            }
        }

        println!("{ans}");
    }
}
