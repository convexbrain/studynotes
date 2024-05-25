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
    let mut lri: Vec<(u32, u32, usize)> = (0..n).map(|i| (tokens.next(), tokens.next(), i)).collect();

    let mut l_gt = BTreeMap::new();
    let mut cnt = 0_usize;
    lri.sort_by_key(|x| x.0);
    for t in lri.iter().rev() {
        cnt += 1;
        l_gt.insert(t.0, cnt);
    }
    debug!(l_gt);

    let mut r_lt = BTreeMap::new();
    let mut cnt = 0_usize;
    lri.sort_by_key(|x| x.1);
    for t in lri.iter() {
        cnt += 1;
        r_lt.insert(t.1, cnt);
    }
    debug!(r_lt);

    lri.sort_by_key(|x| x.2);
    debug!(lri);

    let mut nonint_dup_cnt = 0;
    for t in lri.iter() {
        let ll = t.0;
        let rr = t.1;
        let ii = t.2;
        debug!(ii);

        let r_lt_j = r_lt.range(..ll).rev().next();
        let l_gt_j = l_gt.range((Excluded(rr), Unbounded)).next();
        if r_lt_j.is_some() {
            nonint_dup_cnt += r_lt_j.unwrap().1;
        }
        if l_gt_j.is_some() {
            nonint_dup_cnt += l_gt_j.unwrap().1;
        }
    }

    let ans = (n * (n - 1) / 2) - (nonint_dup_cnt / 2);
    println!("{ans}");
}
