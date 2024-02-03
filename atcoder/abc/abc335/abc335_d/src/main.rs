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

    let mut m = vec![vec![0; n]; n];

    let mut r = 0_isize;
    let mut c = 0_isize;
    let mut st = 0;
    for i in 1..=(n * n - 1) {
        m[r as usize][c as usize] = i;
        match st {
            0 => {
                let mut nr = r;
                let mut nc = c + 1;
                if nc == n as isize || m[nr as usize][nc as usize] > 0 {
                    nr = r + 1;
                    nc = c;
                    st = 1;
                }
                (r, c) = (nr, nc);
            },
            1 => {
                let mut nr = r + 1;
                let mut nc = c;
                if nr == n as isize || m[nr as usize][nc as usize] > 0 {
                    nr = r;
                    nc = c - 1;
                    st = 2;
                }
                (r, c) = (nr, nc);
            },
            2 => {
                let mut nr = r;
                let mut nc = c - 1;
                if nc == -1 || m[nr as usize][nc as usize] > 0 {
                    nr = r - 1;
                    nc = c;
                    st = 3;
                }
                (r, c) = (nr, nc);
            },
            3 => {
                let mut nr = r - 1;
                let mut nc = c;
                if nr == -1 || m[nr as usize][nc as usize] > 0 {
                    nr = r;
                    nc = c + 1;
                    st = 0;
                }
                (r, c) = (nr, nc);
            },
            _ => {
                panic!();
            },
        }
    }

    for r in 0..n {
        for c in 0..n {
            let v = m[r][c];
            if v == 0 {
                print!("T");
            }
            else {
                print!("{v}");
            }

            if c < n - 1 {
                print!(" ");
            }
            else {
                println!();
            }
        }
    }
}
