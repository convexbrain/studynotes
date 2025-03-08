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
    let m: usize = tokens.next();
    let mut b: Vec<i64> = tokens.collect(n);
    let mut w: Vec<i64> = tokens.collect(m);

    b.sort();
    w.sort();

    let mut ans = 0;
    let mut cb = 0;
    let mut cw = 0;
    let mut bi = b.iter().rev().peekable();
    let mut wi = w.iter().rev().peekable();

    while let Some(bb) = bi.peek() {
        if **bb >= 0 {
            ans += bi.next().unwrap();
            cb += 1;
        }
        else {
            break;
        }
    }

    while let Some(ww) = wi.peek() {
        if cw < cb {
            if **ww >= 0 {
                ans += wi.next().unwrap();
                cw += 1;
            }
            else {
                break;
            }
        }
        else {
            if let Some(bb) = bi.peek() {
                if *bb + *ww >= 0 {
                    ans += bi.next().unwrap();
                    ans += wi.next().unwrap();
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }
    }

    println!("{ans}");
}
