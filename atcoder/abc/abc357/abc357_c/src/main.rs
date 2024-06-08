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

fn sub(m: &mut Vec<Vec<bool>>, l: usize, x: usize, y: usize, k: usize) {
    if k == 0 {
        m[y][x] = true;
    }
    else {
        let w = 3_usize.pow(k as u32) / 3;
        sub(m, l, x, y, k - 1);
        sub(m, l, x + w, y, k - 1);
        sub(m, l, x + w * 2, y, k - 1);

        sub(m, l, x, y + w, k - 1);

        sub(m, l, x + w * 2, y + w, k - 1);

        sub(m, l, x, y + w * 2, k - 1);
        sub(m, l, x + w, y + w * 2, k - 1);
        sub(m, l, x + w * 2, y + w * 2, k - 1);

        for yy in (y + w)..(y + w * 2) {
            for xx in (x + w)..(x + w * 2) {
                m[yy][xx] = false;
            }
        }
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let l = 3_usize.pow(n as u32);
    let mut m = vec![vec![false; l]; l];

    sub(&mut m, l, 0, 0, n);

    for y in 0..l {
        for x in 0..l {
            let c = if m[y][x] {'#'} else {'.'};
            print!("{c}");
        }
        println!();
    }
}
