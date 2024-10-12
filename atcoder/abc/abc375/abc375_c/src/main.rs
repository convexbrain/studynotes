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

    let n: usize = tokens.next();
    let mut aa = vec![vec!['#'; n]; n];

    for i in 0..n {
        let a = tokens.next_string();
        for ai in a.char_indices() {
            aa[i][ai.0] = ai.1;
        }
    }

    let mut bb = aa.clone();
    for i in 0..(n/2) {
        match i % 4 {
            0 => {
                for x in i..(n - i) {
                    bb[i][n - 1 - x] = aa[x][i];
                    bb[n - 1 - i][n - 1 - x] = aa[x][n - 1 - i];
                }
                for y in i..(n - i) {
                    bb[y][n - 1 - i] = aa[i][y];
                    bb[y][i] = aa[n - 1 - i][y];
                }
            },
            1 => {
                for x in i..(n - i) {
                    bb[i][n - 1 - x] = aa[n - 1 - i][x];
                    bb[n - 1 - i][n - 1 - x] = aa[i][x];
                }
                for y in i..(n - i) {
                    bb[y][n - 1 - i] = aa[n - 1 - y][i];
                    bb[y][i] = aa[n - 1 - y][n - 1 - i];
                }
            },
            2 => {
                for x in i..(n - i) {
                    bb[i][n - 1 - x] = aa[n - 1 - x][n - 1 - i];
                    bb[n - 1 - i][n - 1 - x] = aa[n - 1 - x][i];
                }
                for y in i..(n - i) {
                    bb[y][n - 1 - i] = aa[n - 1 - i][n - 1 - y];
                    bb[y][i] = aa[i][n - 1 - y];
                }
            },
            3 => {},
            _ => {panic!()},
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", bb[i][j]);
        }
        println!();
    }
}
