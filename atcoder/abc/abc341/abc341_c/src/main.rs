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

    let t = tokens.next_string();

    let mut m = vec![vec![false; w]; h];
    for hi in 0..h {
        let s = tokens.next_string();
        for (wi, c) in s.chars().enumerate() {
            m[hi][wi] = match c {
                '#' => {false},
                '.' => {true},
                _ => {panic!()},
            };
        }
    }
    debug!(m);

    let mut cnt = 0;
    for hi in 1..(h - 1) {
        for wi in 1..(w - 1) {
            let mut hp = hi;
            let mut wp = wi;

            if !m[hp][wp] {
                continue;
            }

            cnt += 1;
            for c in t.chars() {
                match c {
                    'L' => {wp -= 1;},
                    'R' => {wp += 1;},
                    'U' => {hp -= 1;},
                    'D' => {hp += 1;},
                    _ => {panic!()},
                }

                if !m[hp][wp] {
                    cnt -= 1;
                    break;
                }
            }
        }
    }

    println!("{cnt}");
}
