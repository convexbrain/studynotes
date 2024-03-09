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

fn is2(c1: (i64, i64, i64), c2: (i64, i64, i64)) -> i64 {
    let d0 = ( (c1.0 + 7).min(c2.0 + 7) - c1.0.max(c2.0) ).max(0);
    let d1 = ( (c1.1 + 7).min(c2.1 + 7) - c1.1.max(c2.1) ).max(0);
    let d2 = ( (c1.2 + 7).min(c2.2 + 7) - c1.2.max(c2.2) ).max(0);
    d0 * d1 * d2
}

fn is3(c1: (i64, i64, i64), c2: (i64, i64, i64), c3: (i64, i64, i64)) -> i64 {
    let d0 = ( (c1.0 + 7).min(c2.0 + 7).min(c3.0 + 7) - c1.0.max(c2.0).max(c3.0) ).max(0);
    let d1 = ( (c1.1 + 7).min(c2.1 + 7).min(c3.1 + 7) - c1.1.max(c2.1).max(c3.1) ).max(0);
    let d2 = ( (c1.2 + 7).min(c2.2 + 7).min(c3.2 + 7) - c1.2.max(c2.2).max(c3.2) ).max(0);
    d0 * d1 * d2
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let v1: i64 = tokens.next();
    let v2: i64 = tokens.next();
    let v3: i64 = tokens.next();

    for i2 in 0..=(14*14*14) {
        let c2 = (i2 % 15, (i2 / 15) % 15, (i2 / 15 / 15) % 15);
        for i3 in 0..=(14*14*14) {
            let c3 = (i3 % 15, (i3 / 15) % 15, (i3 / 15 / 15) % 15);

            let v12 = is2((7, 7, 7), c2);
            let v13 = is2((7, 7, 7), c3);
            let v23 = is2(c2, c3);

            let v123 = is3((7, 7, 7), c2, c3);

            let v3a = v123;
            let v2a = v12 + v13 + v23 - v3a * 3;
            let v1a = (7 * 7 * 7) * 3 - v2a * 2 - v3a * 3;

            if (v1, v2, v3) == (v1a, v2a, v3a) {
                println!("Yes");
                println!("7 7 7 {} {} {} {} {} {}", c2.0, c2.1, c2.2, c3.0, c3.1, c3.2);
                return;
            }
        }
    }
    println!("No");
}
