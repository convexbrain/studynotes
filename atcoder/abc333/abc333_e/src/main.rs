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
    let tx: Vec<(u8, usize)> = (0..n).map(|_| (tokens.next(), tokens.next::<usize>() - 1)).collect();
    debug!(n, tx);

    let mut mo = vec![0; n];
    let mut po_r = Vec::new();

    for (t, x) in tx.iter().rev() {
        let t = *t;
        let x = *x;

        match t {
            2 => {
                mo[x] += 1;
            },
            1 => {
                if mo[x] > 0 {
                    mo[x] -= 1;
                    po_r.push(true);
                }
                else {
                    po_r.push(false);
                }
            },
            _ => {
                panic!();
            },
        }
    }
    debug!(po_r);
    debug!(mo);

    let mo_sum: usize = mo.iter().sum();
    if mo_sum > 0 {
        println!("-1");
        return;
    }

    let mut po_i = po_r.iter().rev();
    let mut po = 0_usize;
    let mut po_max = 0;
    for (t, _) in tx.iter() {
        let t = *t;
        //let x = *x;

        match t {
            2 => {
                po -= 1;
            },
            1 => {
                if *po_i.next().unwrap() {
                    po += 1;
                    po_max = po.max(po_max);
                }
            },
            _ => {
                panic!();
            },
        }
    }

    println!("{po_max}");
    for t in po_r.iter().rev() {
        print!("{} ", if *t {1} else {0});
    }
    println!("");
}
