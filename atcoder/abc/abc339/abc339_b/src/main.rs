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

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let n: usize = tokens.next();

    let mut m = vec![vec![true; w]; h];

    let mut p = (0, 0);
    let mut d = 0;

    for _ in 0..n {
        if m[p.0][p.1] {
            m[p.0][p.1] = false;
            d = if d < 3 {d + 1} else {0};
        }
        else {
            m[p.0][p.1] = true;
            d = if d > 0 {d - 1} else {3};
        }

        match d {
            0 => {
                p.0 = if p.0 > 0 {p.0 - 1} else {h - 1};
            },
            1 => {
                p.1 = if p.1 < w - 1 {p.1 + 1} else {0};
            },
            2 => {
                p.0 = if p.0 < h - 1 {p.0 + 1} else {0};
            },
            3 => {
                p.1 = if p.1 > 0 {p.1 - 1} else {w - 1};
            },
            _ => {
                panic!();
            }
        }
    }

    for hi in 0..h {
        for wi in 0..w {
            if m[hi][wi] {
                print!(".");
            }
            else {
                print!("#");
            }
        }
        println!();
    }
}
