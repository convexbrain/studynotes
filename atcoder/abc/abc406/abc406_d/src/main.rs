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
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
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
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let n: usize = tokens.next();

    let mut rows = vec![(0, BTreeSet::new()); h];
    let mut cols = vec![(0, BTreeSet::new()); w];

    for _ in 0..n {
        let x: usize = tokens.next();
        let y: usize = tokens.next();
        let x = x - 1;
        let y = y - 1;

        rows[x].0 += 1;
        rows[x].1.insert(y);
        cols[y].0 += 1;
        cols[y].1.insert(x);
    }

    let q: usize = tokens.next();
    let qs: Vec<(usize, usize)> = (0..q).map(|i|
        (tokens.next(), tokens.next())
    ).collect();

    for qi in qs.iter() {
        match qi.0 {
            1 => {
                let x = qi.1 - 1;
                println!("{}", rows[x].0);
                rows[x].0 = 0;
                for y in rows[x].1.iter() {
                    cols[*y].0 -= 1;
                    cols[*y].1.remove(&x);
                }
                rows[x].1.clear();
            },
            2 => {
                let y = qi.1 - 1;
                println!("{}", cols[y].0);
                cols[y].0 = 0;
                for x in cols[y].1.iter() {
                    rows[*x].0 -= 1;
                    rows[*x].1.remove(&y);
                }
                cols[y].1.clear();
            },
            _ => {panic!();}
        }
    }
}
