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
    let k: usize = tokens.next();
    let a: Vec<usize> = tokens.collect(k);
    debug!(n, k, a);

    let mut s = vec![2; n];
    for aa in a.iter() {
        s[aa - 1] = 1;
    }
    debug!(s);

    if n == 1 {
        println!("0");
        return;
    }

    let mut sum = 0;
    let mut t = 0;
    let mut b = n - 1;
    let mut f = true;
    let mut f2 = false;
    while t != b{
        if f {
            if s[t] == 2 {
                s[t] = 0;
                t += 1;
            }
            else if s[t] == 1 {
                if f2 {
                    sum += 1;
                    s[t] -= 1;
                    s[t + 1] -= 1;
                    t += 1;
                    f2 = false;
                }
                else {
                    f = false;
                    f2 = true;
                }
            }
            else {
                t += 1;
            }
        }
        else {
            if s[b] == 2 {
                s[b] = 0;
                b -= 1;
            }
            else if s[b] == 1 {
                if f2 {
                    sum += 1;
                    s[b] -= 1;
                    s[b - 1] -= 1;
                    b -= 1;
                    f2 = false;
                }
                else {
                    f = true;
                    f2 = true;
                }
            }
            else {
                b -= 1;
            }
        }
    }

    println!("{sum}");
}
