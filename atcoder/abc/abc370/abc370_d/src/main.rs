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

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let q: usize = tokens.next();
    let rc: Vec<(usize, usize)> = (0..q).map(|_| (tokens.next(), tokens.next())).collect();

    let mut row = vec![BTreeSet::from_iter(0..w); h];
    let mut col = vec![BTreeSet::from_iter(0..h); w];

    let mut cnt = 0;
    for rci in rc.iter() {
        let r = rci.0 - 1;
        let c = rci.1 - 1;

        if let Some(&cc) = row[r].range((Unbounded, Included(c))).last() {
            row[r].remove(&cc);
            col[cc].remove(&r);
            cnt += 1;

            if cc == c {
                continue;
            }
        }

        if let Some(&cc) = row[r].range((Excluded(c), Unbounded)).next() {
            row[r].remove(&cc);
            col[cc].remove(&r);
            cnt += 1;
        }

        if let Some(&rr) = col[c].range((Unbounded, Excluded(r))).last() {
            col[c].remove(&rr);
            row[rr].remove(&c);
            cnt += 1;
        }

        if let Some(&rr) = col[c].range((Excluded(r), Unbounded)).next() {
            col[c].remove(&rr);
            row[rr].remove(&c);
            cnt += 1;
        }
        debug!(cnt);
    }
    println!("{}", h * w - cnt);
}
