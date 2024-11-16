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

    let s = tokens.next_string(); // String
    let q: usize = tokens.next();

    let sv: Vec<char> = s.chars().collect();

    for _ in 0..q {
        let k: usize = tokens.next();
        let k = k - 1;

        let kp = k / sv.len();
        let kq = k % sv.len();
        //debug!(kp, kq);
        if kp > 0 {
            let mut t = true;
            let mut kp = kp;
            loop {
                let kpe = kp.ilog2();
                let kpr = kp - (1 << kpe);
                debug!(kp, kpe, kpr);

                if kpr > 0 {
                    if kpr < (1 << kpe) / 2 {
                        kp = (1 << kpe) / 2 + kpr;
                    }
                    else {
                        kp = (1 << kpe) / 2 + kpr - (1 << kpe) / 2;
                        t = !t;
                    }
                }
                else {
                    if t {
                        if sv[kq].is_uppercase() {
                            print!("{} ", sv[kq].to_lowercase());
                        }
                        else {
                            print!("{} ", sv[kq].to_uppercase());
                        }
                    }
                    else {
                        print!("{} ", sv[kq]);
                    }
                    break;
                }
            }
        }
        else {
            print!("{} ", sv[kq]);
        }
    }
    println!();
}
