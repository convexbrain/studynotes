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
    let q: Vec<u32> = tokens.collect(n);
    let a: Vec<u32> = tokens.collect(n);
    let b: Vec<u32> = tokens.collect(n);
    debug!(n, q, a, b);

    let mut aa = Vec::new();
    let mut bb = Vec::new();
    let mut flag_ex = true;
    for i in 0..n {
        if a[i] == 0 {
            aa.push(u32::MAX);
        }
        else {
            aa.push(q[i] / a[i]);
        }
        if b[i] == 0 {
            bb.push(u32::MAX);
        }
        else {
            bb.push(q[i] / b[i]);
        }
        if a[i] > 0 && b[i] > 0 {
            flag_ex = false;
        }
    }
    debug!(aa, bb, flag_ex);

    aa.sort();
    let aa_min = aa[0];
    bb.sort();
    let bb_min = bb[0];
    if flag_ex {
        println!("{}", aa_min + bb_min);
    }
    else {
        let (a_max, a, b) = if aa_min < bb_min {
            (bb_min, b, a)
        }
        else {
            (aa_min, a, b)
        };

        let mut ans = 0;
        for anum in 0..=a_max {
            let mut bnum = u32::MAX;
            for ni in 0..n {
                if b[ni] > 0 {
                    let bn = (q[ni] - a[ni] * anum) / b[ni];
                    bnum = bn.min(bnum);
                }
            }

            ans = ans.max(anum + bnum);
        }
        println!("{ans}");
    }
}
