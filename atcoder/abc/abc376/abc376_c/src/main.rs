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
    let mut a: Vec<u32> = tokens.collect(n);
    let mut b: Vec<u32> = tokens.collect(n - 1);

    a.sort();
    b.sort();

    let mut j = n - 2;
    let mut ans = None;
    for i in (0..n).rev() {
        if let Some(_) = ans {
            if a[i] > b[j] {
                debug!();
                println!("-1");
                return;
            }
            else {
                if j > 0 {
                    j -= 1;
                }
                else {
                    if i > 1 {
                        debug!();
                        println!("-1");
                        return;
                    }
                }
            }
        }
        else {
            if a[i] > b[j] {
                ans = Some(a[i]);
                debug!(i, j, ans);
            }
            else {
                if j > 0 {
                    j -= 1;
                }
                else {
                    if i > 1 {
                        debug!(i, j);
                        println!("-1");
                        return;
                    }
                    else {
                        println!("{}", a[0]);
                        return;
                    }
                }
            }
        }
    }
    println!("{}", ans.unwrap());
}
