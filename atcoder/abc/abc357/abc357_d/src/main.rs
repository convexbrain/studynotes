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

fn sub2(p: u64, n: u64) -> u64 {
    let q = 998244353;

    if n == 1 {
        p % q
    }
    else if n % 2 == 0 {
        sub2((p * p) % q, n / 2) % q
    }
    else {
        (sub2(p, n - 1) * p) % q
    }
}

fn sub(memo: &mut BTreeMap<u64, u64>, n: u64, l: u64) -> u64 {
    let q = 998244353;

    if n == 1 {
        1
    }
    else if(memo.contains_key(&n)) {
        memo[&n]
    }
    else {
        let n1 = n / 2;
        let n2 = n - n1;

        let s1 = sub(memo, n1, l);
        let mut s2 = sub(memo, n2, l);
        //debug!(n1, s1, n2, s2);
        s2 *= sub2(sub2(10, l), n1);
        s2 %= q;
        //debug!(n1, s1, n2, s2);

        let s = (s1 + s2) % q;
        memo.insert(n, s);
        s
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: u64 = tokens.next();
    let l = format!("{n}").chars().count() as u64;
    debug!(n, l);

    let q = 998244353;

    let mut memo = BTreeMap::new();
    let s = sub(&mut memo, n, l);
    let nq = n % q;
    let a = (s * nq) % q;

    println!("{a}");
}
