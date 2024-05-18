use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    rc::*, cell::*,
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

struct Tokens<'a>(std::str::SplitWhitespace<'a>);

#[allow(dead_code)]
impl<'a> Tokens<'a> {
    fn new(buf: &'a mut String) -> Self {
        std::io::stdin().read_to_string(buf).unwrap();
        Tokens(buf.split_whitespace())
    }
    fn release(self) -> std::str::SplitWhitespace<'a> {
        self.0
    }
    fn next_string(&mut self) -> String {
        self.0.next().unwrap().to_string()
    }
    fn next_bytes(&mut self) -> Vec<u8> {
        self.0.next().unwrap().as_bytes().to_vec()
    }
    fn next<T>(&mut self) -> T
    where T: std::str::FromStr, T::Err: std::fmt::Debug {
        self.0.next().unwrap().parse().unwrap()
    }
    fn collect<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<T> {
        (0..n).map(|_| self.next()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.next())).collect()
    }
}

//#############################################################################

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let a: i64 = tokens.next();
    let b: i64 = tokens.next();
    let c: i64 = tokens.next();
    let d: i64 = tokens.next();

    let w = c - a;
    let h = d - b;

    let k = w / 4;
    let p = w % 4;
    let l = h / 2;
    let q = h % 2;

    let e = a + 4 * k;
    let f = b + 2 * l;

    let mut ans = 8 * k * l;

    if q == 1 {
        ans += 4 * k;
    }

    {
        let mut t = 0;
        for x in e..(e + p) {
            let xm = (4 + x % 4) % 4;
            t += match xm {
                0 => {3},
                1 => {3},
                2 => {1},
                3 => {1},
                _ => {debug!(xm); panic!()},
            };
        }
        ans += t * l;
    }

    {
        let mut t = 0;
        for x in e..(e + p) {
            for y in f..(f + q) {
                let xm = (4 + x % 4) % 4;
                let ym = (2 + y % 2) % 2;
                t += match (xm, ym) {
                    (0, 0) => {2},
                    (1, 0) => {1},
                    (2, 0) => {0},
                    (3, 0) => {1},
                    (0, 1) => {1},
                    (1, 1) => {2},
                    (2, 1) => {1},
                    (3, 1) => {0},
                    _ => {debug!(xm, ym); panic!()},
                };
            }
        }
        ans += t;
    }

    println!("{ans}");
}
