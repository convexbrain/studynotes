use std::{prelude::rust_2021::*, usize};
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
    let a = tokens.next_bytes();

    let mut aa = Vec::new();
    for ai in a.iter() {
        aa.push((*ai, 1));
    }

    for _ in 0..n {
        let mut aaa = Vec::new();
        for i in 0..(aa.len() / 3) {
            let i0 = i * 3;
            let i1 = i * 3 + 1;
            let i2 = i * 3 + 2;
            if aa[i0].0 == aa[i1].0 && aa[i1].0 == aa[i2].0 {
                let mut s = Vec::new();
                s.push(aa[i0].1);
                s.push(aa[i1].1);
                s.push(aa[i2].1);
                s.sort();
                aaa.push((aa[i0].0, s[0] + s[1]));
            } else {
                let mut m = Vec::new();
                m.push(aa[i0].0);
                m.push(aa[i1].0);
                m.push(aa[i2].0);
                m.sort();
                let mut c = usize::MAX;
                if m[1] == aa[i0].0 {
                    c = c.min(aa[i0].1);
                }
                if m[1] == aa[i1].0 {
                    c = c.min(aa[i1].1);
                }
                if m[1] == aa[i2].0 {
                    c = c.min(aa[i2].1);
                }
                aaa.push((m[1], c));
            }
        }
        aa = aaa;
        debug!(aa);
    }
    println!("{}", aa[0].1);
}
