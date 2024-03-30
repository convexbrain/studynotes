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

    let a: usize = tokens.next();
    let b: usize = tokens.next();
    let c: u64 = tokens.next();

    let pc = {
        let mut cc = c;
        let mut cnt = 0_usize;
        while cc > 0 {
            if cc & 1 != 0 {
                cnt += 1;
            }
            cc >>= 1;
        }
        cnt
    };

    if pc > a + b {
        println!("-1");
        return;
    }

    let k2 = a + b - pc;
    if k2 & 1 == 1 {
        println!("-1");
        return;
    }

    let mut k = k2 / 2;

    let mut x = 0;
    let mut y = 0;
    {
        let mut ak = a - k;
        let mut bk = b - k;
        let mut cc = c;
        let mut bit = 1_u64;
        while cc > 0 || k > 0 {
            if cc & 1 != 0 {
                if ak > 0 {
                    x |= bit;
                    ak -= 1;
                }
                else if bk > 0 {
                    y |= bit;
                    bk -= 1;
                }
                else {
                    debug!(x, y);
                    println!("-1");
                    return;
                }
            }
            else if k > 0 {
                x |= bit;
                y |= bit;
                k -= 1;
            }
            cc >>= 1;
            bit <<= 1;
        }
    }
    println!("{x} {y}");
}
