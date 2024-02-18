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
    let q: usize = tokens.next();

    let s = tokens.next_bytes();
    debug!(s);

    let mut set = BTreeSet::new();

    let mut pc = s[0];
    for (i, c) in s.iter().enumerate() {
        if i > 0 && pc == *c {
            set.insert(i);
        }
        pc = *c;
    }

    for _ in 0..q {
        debug!(set);
        let t: usize = tokens.next();
        let l: usize = tokens.next();
        let r: usize = tokens.next();

        match t {
            1 => {
                if 0 < l - 1 && l - 1 < n {
                    if set.contains(&(l - 1)) {
                        set.remove(&(l - 1));
                    }
                    else {
                        set.insert(l - 1);
                    }
                }
    
                if 0 < r && r < n {
                    if set.contains(&r) {
                        set.remove(&r);
                    }
                    else {
                        set.insert(r);
                    }
                }    
            },
            2 => {
                if let Some(_) = set.range(l..r).next() {
                    println!("No");
                }
                else {
                    println!("Yes");
                }
            },
            _ => {panic!();}
        }
    }
}
