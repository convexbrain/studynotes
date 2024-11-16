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
    let k: usize = tokens.next();
    let s = tokens.next_string(); // String

    let s0: Vec<&str> = s.split('1').filter(|x| x.len() > 0).collect();
    let s1: Vec<&str> = s.split('0').filter(|x| x.len() > 0).collect();
    debug!(s0, s1);

    let mut f0 = if s.chars().next().unwrap() == '0' {true} else {false};

    let mut i0 = 0;
    let mut i1 = 0;
    loop {
        let mut e = true;
        if f0 {
            if i0 < s0.len() {
                print!("{}", s0[i0]);
                e = false;
            }
            i0 += 1;
            f0 = false;
        }
        else {
            if i1 < s1.len() {
                e = false;
                if i1 + 1 == k - 1 {
                    print!("{}", s1[i1]);
                    print!("{}", s1[i1 + 1]);
                }
                else if i1 + 1 != k {
                    print!("{}", s1[i1]);
                }
            }
            i1 += 1;
            f0 = true;
        }
        if e {
            break;
        }
    }
}
