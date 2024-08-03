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
    let a: Vec<i32> = tokens.collect(n);
    let bk: Vec<(i32, usize)> = (0..q).map(|i| (tokens.next(), tokens.next())).collect();

    let mut a = a;
    a.sort();

    for (b, k) in bk.iter() {
        let b = *b;
        let k = *k;

        let r = a.binary_search(&b);
        let i = r.unwrap_or_else(|x| x);
        let mut iu = i as isize;
        let mut id = (i - 1) as isize;
        let mut iv = 0;
        for _ in 0..k {
            if iu >= a.len() as isize {
                let y = b - a[id as usize];
                iv = y;
                id -= 1;
            }
            else if id < 0 {
                let x = a[iu as usize] - b;
                iv = x;
                iu += 1;
            }
            else {
                let x = a[iu as usize] - b;
                let y = b - a[id as usize];
                if x < y {
                    iv = x;
                    iu += 1;
                }
                else {
                    iv = y;
                    id -= 1;
                }
            }
        }
        println!("{iv}");
    }
}
