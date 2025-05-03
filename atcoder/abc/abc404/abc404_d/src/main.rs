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

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let m: usize = tokens.next();
    let c: Vec<u64> = tokens.collect(n);
    let mut z = vec![Vec::new(); n];
    for i in 0..m {
        let k: usize = tokens.next();
        let a: Vec<usize> = tokens.collect(k);
        for ai in a.iter() {
            z[ai - 1].push(i);
        }
    }
    debug!(z);

    let mut mon_min = u64::MAX;
    for x in 0..(3_usize.pow(n as u32)) {
        let mut aa = vec![0; m];
        let mut mon = 0;
        let mut xx = x;
        for i in 0..n {
            let xm = xx % 3;
            for za in z[i].iter() {
                aa[*za] += xm;
            }
            mon += c[i] * xm as u64;
            xx /= 3;
        }
        let mut flag = true;
        for ai in aa.iter() {
            if *ai < 2 {
                flag = false;
                break;
            }
        }
        if flag {
            mon_min = mon_min.min(mon);
        }
    }
    println!("{mon_min}");
}
