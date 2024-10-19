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
    let q: usize = tokens.next();
    let ht: Vec<(char, usize)> = (0..q).map(|_| (tokens.next(), tokens.next())).collect();

    let mut cnt = 0;
    let mut l = 1;
    let mut r = 2;

    for (h, t) in ht.iter() {
        let h = *h;
        let t = *t;
        match h {
            'L' => {
                if l < r {
                    if r < t {
                        cnt += n - t + l;
                    }
                    else if t < l {
                        cnt += l - t;
                    }
                    else {
                        cnt += t - l;
                    }
                } 
                else {
                    if t < r {
                        cnt += n - l + t;
                    }
                    else if l < t {
                        cnt += t - l;
                    }
                    else {
                        cnt += l - t;
                    }
                }
                l = t;
            },
            'R' => {
                if l < r {
                    if r < t {
                        cnt += t - r;
                    }
                    else if t < l {
                        cnt += n - r + t;
                    }
                    else {
                        cnt += r - t;
                    }
                } 
                else {
                    if t < r {
                        cnt += r - t;
                    }
                    else if l < t {
                        cnt += n - t + r;
                    }
                    else {
                        cnt += t - r;
                    }
                }
                r = t;
            },
            _ => {panic!()}
        }
    }
    println!("{cnt}");
}
