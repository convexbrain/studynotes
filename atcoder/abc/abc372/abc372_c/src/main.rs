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
    let s = tokens.next_string(); // String
    let xc: Vec<(usize, char)> = (0..q).map(|_| (tokens.next(), tokens.next())).collect();

    let mut idx: BTreeSet<usize> = s.match_indices("ABC").map(|x| x.0).collect();
    debug!(idx);

    let mut ss: Vec<char> = s.chars().collect();
    for (x, c) in xc.iter() {
        let x = *x - 1;
        let c = *c;
        for d in 0..3 {
            if x >= d && idx.contains(&(x - d)) {
                idx.remove(&(x - d));
            }
        }
        ss[x] = c;
        debug!(ss);
        match c {
            'A' => {
                if x + 2 < n && ss[x + 1] == 'B' && ss[x + 2] == 'C' {
                    idx.insert(x);
                }
            },
            'B' => {
                if x > 0 && x + 1 < n && ss[x - 1] == 'A' && ss[x + 1] == 'C' {
                    idx.insert(x - 1);
                }
            },
            'C' => {
                if x > 1 && ss[x - 2] == 'A' && ss[x - 1] == 'B' {
                    idx.insert(x - 2);
                }
            },
            _ => {},
        }
        println!("{}", idx.len());
    }
}
