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

    let mut ca = Vec::new();

    for _ in 0..m {
        let k: usize = tokens.next();
        let c: u64 = tokens.next();
        let a: BTreeSet<usize> = tokens.collect(k);

        ca.push((c, a));
    }

    ca.sort_by_key(|x| x.0);
    debug!(ca);

    let mut set = BTreeSet::new();
    let mut score = 0;

    for cai in ca.iter() {
        if cai.1.is_disjoint(&set) {
        }
        else {
            let diff: BTreeSet<usize> = cai.1.difference(&set).copied().collect();
            score += cai.0 * (diff.len() as u64);

            for di in diff.iter() {
                set.insert(*di);
            }
        }
    }

    for cai in ca.iter() {
        if cai.1.is_disjoint(&set) {
            score += cai.0 * (cai.1.len() as u64 - 1);

            for di in cai.1.iter() {
                set.insert(*di);
            }
        }
        else {
            let diff: BTreeSet<usize> = cai.1.difference(&set).copied().collect();
            score += cai.0 * (diff.len() as u64);

            for di in diff.iter() {
                set.insert(*di);
            }
        }
    }

    debug!(set);

    if set.len() < n {
        println!("-1");
    }
    else {
        println!("{score}");
    }
}
