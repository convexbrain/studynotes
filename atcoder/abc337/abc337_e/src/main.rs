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
    let mut stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: usize = buf.split_whitespace().next().unwrap().parse().unwrap();

    let m = (n + 1) / 2;
    println!("{m}");
    stdout.flush().unwrap();
    for i in 0..m {
        println!("2 {} {}", i + 1, i + 2);
        stdout.flush().unwrap();
    }

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    //println!("{buf}");
    let sb = buf.as_bytes().to_vec();
    let s = buf;
    let ans = if let Some(idx) = s.find('1') {
        if idx == 0 {
            if sb[1] == b'0' {
                0 + 1
            }
            else {
                1 + 1
            }
        }
        else {
            idx + 1 + 1
        }
    }
    else {
        n
    };
    println!("{ans}");
    stdout.flush().unwrap();
}
