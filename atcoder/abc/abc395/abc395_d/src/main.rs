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

    let mut pig_box: Vec<usize> = (0..n).collect();
    let mut nes_box: Vec<usize> = (0..n).collect();
    let mut box_nes: Vec<usize> = (0..n).collect();

    for _ in 0..q {
        let t: usize = tokens.next();
        match t {
            1 => {
                let a: usize = tokens.next();
                let b: usize = tokens.next();
                let a = a - 1;
                let b = b - 1;
                let box_b = nes_box[b];
                pig_box[a] = box_b;
            },
            2 => {
                let a: usize = tokens.next();
                let b: usize = tokens.next();
                let a = a - 1;
                let b = b - 1;
                let box_a = nes_box[a];
                let box_b = nes_box[b];
                nes_box[a] = box_b;
                nes_box[b] = box_a;
                box_nes[box_a] = b;
                box_nes[box_b] = a;
            },
            3 => {
                let a: usize = tokens.next();
                let a = a - 1;
                let nes_a = box_nes[pig_box[a]] + 1;
                println!("{nes_a}");
            },
            _ => {panic!();}
        }
    }
}
