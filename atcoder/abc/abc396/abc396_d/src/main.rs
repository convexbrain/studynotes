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

fn sub(g: &[BTreeSet<(usize, u64)>], v: &mut BTreeSet<usize>, i: usize, x: u64, min_x: &mut u64) {
    let n = g.len();
    if i == n - 1 {
        *min_x = x.min(*min_x);
    }
    else if !v.contains(&i) {
        v.insert(i);

        for c in g[i].iter() {
            let nx = x ^ c.1;
            sub(g, v, c.0, nx, min_x);
        }

        v.remove(&i);
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let m: usize = tokens.next();

    let mut g = vec![BTreeSet::new(); n];
    for _ in 0..m {
        let u: usize = tokens.next();
        let v: usize = tokens.next();
        let w: u64 = tokens.next();

        let u = u - 1;
        let v = v - 1;
        g[u].insert((v, w));
        g[v].insert((u, w));
    }

    let mut v = BTreeSet::new();
    let mut min_x = u64::MAX;
    sub(&g, &mut v, 0, 0, &mut min_x);
    println!("{}", min_x);
}
