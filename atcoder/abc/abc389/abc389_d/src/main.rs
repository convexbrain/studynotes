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

    let r: u64 = tokens.next();

    let mut n = 0;
    let mut n2 = 0;
    let mut n4 = 0;
    for i in 0.. {
        debug!(i);
        let r2 = 2 * r;
        let x2 = 2 * i + 1;
        let y2 = 2 * 0 + 1;
        if x2 * x2 + y2 * y2 > r2 * r2 {
            break;
        }

        let mut lt = 0;
        let mut rt = r;
        while lt + 1 < rt {
            let ct = (lt + rt) / 2;

            let y2 = 2 * ct + 1;
            if x2 * x2 + y2 * y2 <= r2 * r2 {
                lt = ct;
            }
            else {
                rt = ct;
            }
        }

        let z = lt + 1;
        debug!(z);
        if z > 0 {
            n += z;
            if i == 0 {
                n2 += z - 1;
                n4 += 1;
            }
            else {
                n2 += 1;
            }
        }
        debug!(n, n2, n4);
    }
    let a = (n - n2 - n4) * 4 + n2 * 2 + n4;
    println!("{}", a);
}
