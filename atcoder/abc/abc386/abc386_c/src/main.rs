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

    let k: usize = tokens.next();
    let s = tokens.next_bytes();
    let t = tokens.next_bytes();
    let sn = s.len();
    let tn = t.len();

    if sn < tn {
        let mut si = 0;
        let mut diff = false;
        for ti in 0..tn {
            if si < sn {
                if s[si] == t[ti] {
                    si += 1;
                }
                else {
                    if !diff {
                        diff = true;
                    }
                    else {
                        println!("No");
                        return;
                    }
                }
            }
            else {
                if !diff {
                    diff = true;
                }
                else {
                    println!("No");
                    return;
                }
            }
        }
        println!("Yes");
    }
    else if sn > tn {
        let mut ti = 0;
        let mut diff = false;
        for si in 0..sn {
            if ti < tn {
                if s[si] == t[ti] {
                    ti += 1;
                }
                else {
                    if !diff {
                        diff = true;
                    }
                    else {
                        println!("No");
                        return;
                    }
                }
            }
            else {
                if !diff {
                    diff = true;
                }
                else {
                    println!("No");
                    return;
                }
            }
        }
        println!("Yes");
    }
    else {
        let mut diff = false;
        for i in 0..sn {
            if s[i] != t[i] {
                if !diff {
                    diff = true;
                }
                else {
                    println!("No");
                    return;
                }
            }
        }
        println!("Yes");
    }
}
