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

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let s: (isize, isize) = (tokens.next(), tokens.next());
    let mut c = vec![vec![false; w]; h];
    for ri in 0..h {
        let row = tokens.next_string(); // String
        for ci in row.chars().enumerate() {
            c[ri][ci.0] = ci.1 == '#';
        }
    }
    debug!(c);
    let x = tokens.next_string(); // String

    let mut cs = (s.0 - 1, s.1 - 1);
    for xi in x.chars() {
        debug!(cs);
        let ns = match xi {
            'L' => { (cs.0, cs.1 - 1) },
            'R' => { (cs.0, cs.1 + 1) },
            'U' => { (cs.0 - 1, cs.1) },
            'D' => { (cs.0 + 1, cs.1) },
            _ => {panic!()},
        };
        if ns.0 < 0 || ns.0 > (h - 1) as isize || ns.1 < 0 || ns.1 > (w - 1) as isize {
            continue;
        }
        if c[ns.0 as usize][ns.1 as usize] {
            continue;
        }
        cs = ns;
    }

    println!("{} {}", cs.0 + 1, cs.1 + 1);
}
