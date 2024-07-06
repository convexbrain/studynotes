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
    let t: i64 = tokens.next();
    let s = tokens.next_bytes(); // Vec<u8>
    let x: Vec<i64> = tokens.collect(n);

    let mut ls1 = Vec::new();
    let mut rs1 = Vec::new();
    let mut ls0 = Vec::new();
    let mut rs0 = Vec::new();
    let mut n1 = 0_usize;
    let mut n0 = 0_usize;
    for i in 0..n {
        if s[i] == b'1' {
            let (l, r) = (x[i], x[i] + t);
            ls1.push(l);
            rs1.push(r);
            n1 += 1;
        }
        else {
            let (l, r) = (x[i] - t, x[i]);
            ls0.push(l);
            rs0.push(r);
            n0 += 1;
        };
    }
    ls1.sort();
    rs1.sort();
    ls0.sort();
    rs0.sort();
    debug!(ls1, rs1, ls0, rs0);

    let mut ls0g = BTreeMap::new();
    let mut rs0l = BTreeMap::new();

    let mut c: usize = 0;
    for i in ls0.iter().rev() {
        c += 1;
        ls0g.entry(*i).and_modify(|e| *e = c).or_insert(c);
    }
    let mut c: usize = 0;
    for i in rs0.iter() {
        c += 1;
        rs0l.entry(*i).and_modify(|e| *e = c).or_insert(c);
    }
    debug!(ls0g, rs0l);

    let mut ans = 0;
    for i in 0..n1 {
        let mut int = n0;
        let (l, r) = (ls1[i], rs1[i]);
        debug!(l, r);
        if let Some(e) = ls0g.range((Excluded(r), Unbounded)).next() {
            int -= e.1;
            debug!(e);
        }
        if let Some(e) = rs0l.range((Unbounded, Excluded(l))).rev().next() {
            int -= e.1;
            debug!(e);
        }
        ans += int;
        debug!(int, ans);
    }

    println!("{ans}");
}
