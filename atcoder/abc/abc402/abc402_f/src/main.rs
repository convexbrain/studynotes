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

fn mod_pow<N>(mut x: N, mut p: N, m: N) -> N
where N: Default + Ord + BitAnd<Output=N> + ShrAssign + Mul<Output=N> + Rem<Output=N> + SubAssign + Copy + Div<Output=N>
{
    let zero = N::default();
    let one = m / m;

    if p == zero {
        return one;
    }

    let mut k = one;

    while p > one {
        if p & one == zero {
            x = (x * x) % m;
            p >>= one;
        }
        else {
            k = (k * x) % m;
            p -= one;
        }
    }
    (k * x) % m
}

//#############################################################################

fn sub(aa: &Vec<Vec<u64>>, i: usize, j: usize, s: u64, m: u64, n: usize) -> u64 {
    let s = (s + aa[i][j]) % m;

    if i == n - 1 && j == n - 1 {
        s
    }
    else {
        let mut ss = 0;
        if i < n - 1 {
            let s0 = sub(aa, i + 1, j, s, m, n);
            ss = ss.max(s0);
        }
        if j < n - 1 {
            let s1 = sub(aa, i, j + 1, s, m, n);
            ss = ss.max(s1);
        }
        ss
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let m: u64 = tokens.next();
    let mut aa = Vec::new();
    for i in 0..n {
        let mut a: Vec<u64> = tokens.collect(n);
        for (j, ai) in a.iter_mut().enumerate() {
            let p = mod_pow(10, (2 * n - 2 - (i + j)) as u64, m);
            *ai = (p * *ai) % m;
        }
        aa.push(a);
    }

    let ans = sub(&aa, 0, 0, 0, m, n);
    println!("{ans}");
}
