use std::{hash::Hash, prelude::rust_2021::*};
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

    let n: usize = tokens.next();
    let c: Vec<usize> = tokens.collect(n - 1);
    let mut a: VecDeque<u8> = tokens.collect(n - 1);

    a.push_front(0);
    while a.back().unwrap() == &0 {
        a.pop_back();
    }

    let mut vis = BTreeSet::new();

    let mut que = BinaryHeap::new();
    que.push((Reverse(0), a));
    while let Some(q) = que.pop() {
        let l = q.1.len();
        if l == 1 {
            println!("{}", q.0.0);
            return;
        }
        else if !vis.contains(&q.1) {
            vis.insert(q.1.clone());
            for i in 0..c[l - 2] {
                let mut na = q.1.clone();
                na[l - 1] = 0;
                na[l - 2 - i] = 1;
                while na.back().unwrap() == &0 {
                    na.pop_back();
                }
                que.push((Reverse(q.0.0 + 1), na));
            }
        }
    }
}
