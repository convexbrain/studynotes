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

fn sub(s: (i64, i64), t: (i64, i64)) -> i64 {
    let d1 = s.1.abs_diff(t.1) as i64;
    let l = s.0 - d1 / 2;
    let r = s.0 + d1 / 2;
    //debug!(d1, l, r);

    if t.0 < l {
        d1 + l - t.0
    }
    else if r < t.0 {
        d1 + t.0 - r
    }
    else {
        d1
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let sx: i64 = tokens.next();
    let sy: i64 = tokens.next();
    let tx: i64 = tokens.next();
    let ty: i64 = tokens.next();

    let s = if sy % 2 == 0 {
        (sx / 2, sy)
    }
    else {
        ((sx + 1) / 2, sy)
    };

    let t = if ty % 2 == 0 {
        (tx / 2, ty)
    }
    else {
        ((tx + 1) / 2, ty)
    };

    let ans = if (t.1 - s.1) % 2 == 0 {
        debug!(s, t);
        sub(s, t)
    }
    else {
        debug!(s, t);
        let s1 = (s.0, s.1 + 1);
        let s2 = (s.0 + 1, s.1 + 1);
        let s3 = (s.0, s.1 - 1);
        let s4 = (s.0 + 1, s.1 - 1);
        debug!(s1, s2, s3, s4);

        let mut v = Vec::new();
        v.push(sub(s1, t) + 1);
        v.push(sub(s2, t) + 1);
        v.push(sub(s3, t) + 1);
        v.push(sub(s4, t) + 1);
        v.sort();
        debug!(v);
        v[0]
    };
    
    println!("{ans}");
}
