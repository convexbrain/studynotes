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

    let n: u32 = tokens.next();
    let q: usize = tokens.next();
    debug!(n, q);

    let xyzw: Vec<(usize, usize, usize, u64)> = (0..q).map(|_| (
        tokens.next(),
        tokens.next(),
        tokens.next(),
        tokens.next(),
    )).collect();
    debug!(xyzw);

    let mut ans = 1_u64;
    for bit in 0..60 {
        let mut cnt = 0;
        for mask in 0..(2_u32.pow(n)) {
            let mut flag = true;
            for (x, y, z, w) in xyzw.iter() {
                let bx = (mask >> (x - 1)) & 1;
                let by = (mask >> (y - 1)) & 1;
                let bz = (mask >> (z - 1)) & 1;
                let bw = (w >> bit) & 1;
                if bx | by | bz != bw as u32 {
                    flag = false;
                    break;
                }
            }
            if flag {
                cnt += 1;
            }
        }

        ans = (ans * cnt) % 1000000007;
    }

    println!("{ans}");
}
