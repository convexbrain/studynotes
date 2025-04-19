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
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
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
}

//#############################################################################

fn sub(aa: &Vec<Vec<char>>, i: usize, j: usize, mut s: String, m: u64, n: usize) -> u64 {
    s.push(aa[i][j]);
    if i == n - 1 && j == n - 1 {
        let ss: u64 = s.parse().unwrap();
        ss % m
    }
    else {
        let mut ss = 0;
        if i < n - 1 {
            let s0 = sub(aa, i + 1, j, s.clone(), m, n);
            ss = ss.max(s0);
        }
        if j < n - 1 {
            let s1 = sub(aa, i, j + 1, s.clone(), m, n);
            ss = ss.max(s1);
        }
        ss
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let m: u64 = tokens.next();
    let mut aa = Vec::new();
    for _ in 0..n {
        let a: Vec<char> = tokens.collect(n);
        aa.push(a);
    }

    let ans = sub(&aa, 0, 0, String::new(), m, n);
    println!("{ans}");
}
