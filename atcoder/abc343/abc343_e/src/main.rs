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

fn is(
    c1: (i64, i64, i64), s1: (i64, i64, i64),
    c2: (i64, i64, i64), s2: (i64, i64, i64))
-> ((i64, i64, i64), (i64, i64, i64)) {

    for p in 0..8 {
        let d0 = if p & 1 == 0 { c1.0 + s1.0 - c2.0 } else { c2.0 + s2.0 - c1.0 };
        let d1 = if p & 2 == 0 { c1.1 + s1.1 - c2.1 } else { c2.1 + s2.1 - c1.1 };
        let d2 = if p & 4 == 0 { c1.2 + s1.2 - c2.2 } else { c2.2 + s2.2 - c1.2 };

        let c0a = if p & 1 == 0 { c2.0 } else { c1.0 };
        let c1a = if p & 2 == 0 { c2.1 } else { c1.1 };
        let c2a = if p & 4 == 0 { c2.2 } else { c1.2 };

        if d0 > 0 && d1 > 0 && d2 > 0 {
            return ((c0a, c1a, c2a), (d0, d1, d2));
        }
    }

    ((0, 0, 0), (0, 0, 0))
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let v1: i64 = tokens.next();
    let v2: i64 = tokens.next();
    let v3: i64 = tokens.next();

    for i2 in 0..(14*14*14) {
        let c2 = (i2 % 14, (i2 / 14) % 14, (i2 / 14 / 14) % 14);
        for i3 in 0..(14*14*14) {
            let c3 = (i3 % 14, (i3 / 14) % 14, (i3 / 14 / 14) % 14);

            let is12 = is((7, 7, 7), (7, 7, 7), c2, (7, 7, 7));
            let is13 = is((7, 7, 7), (7, 7, 7), c3, (7, 7, 7));
            let is23 = is(c2, (7, 7, 7), c3, (7, 7, 7));

            let v12 = is12.1.0 * is12.1.1 * is12.1.2;
            let v13 = is13.1.0 * is13.1.1 * is13.1.2;
            let v23 = is23.1.0 * is23.1.1 * is23.1.2;

            let is123 = is((7, 7, 7), (7, 7, 7), is23.0, is23.1);

            let v3a = is123.1.0 * is123.1.1 * is123.1.2;
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
