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
    
    let n: usize = tokens.next();
    let p: Vec<u32> = tokens.collect(n);

    let mut u = Vec::new();
    let mut uu = 0;
    u.push(0);
    for i in 0..(n - 1) {
        if p[i] < p[i + 1] {
            uu += 1;
        }
        u.push(uu);
    }
    u.push(uu);
    debug!(u);

    let mut m = Vec::new();
    for i in 1..(n - 1) {
        if p[i - 1] < p[i] && p[i] > p[i + 1] {
            m.push((i, true));
        }
        if p[i - 1] > p[i] && p[i] < p[i + 1] {
            m.push((i, false));
        }
    }
    debug!(m);

    let mut cnt = 0;
    if m.len() > 0 {
        for k in 0..(m.len() - 1) {
            if (m[k].1 && !m[k + 1].1) || (!m[k].1 && m[k + 1].1) {
                let il = if k > 0 {m[k - 1].0} else {0};
                let ir = m[k].0 - 1;
                let ii = if il <= ir {u[ir + 1] - u[il]} else {0};

                let jl = m[k + 1].0 + 1;
                let jr = if k < m.len() - 2 {m[k + 2].0} else {n - 1};
                let jj = if jl <= jr {jr - jl + 1} else {0};

                debug!(k, il, ir, jl, jr, ii, jj);

                cnt += ii * jj;
            }
        }
    }
    println!("{cnt}");
}
