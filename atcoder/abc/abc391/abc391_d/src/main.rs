use std::{prelude::rust_2021::*, usize};
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
    let w: usize = tokens.next();
    let mut xy = Vec::new();
    for _ in 0..n {
        let x: usize = tokens.next();
        let y: usize = tokens.next();
        let x = x - 1;
        let y = y - 1;
        xy.push((x, y));
    }
    let xy = xy;

    let mut wbl = vec![Vec::new(); w];
    for i in 0..n {
        wbl[xy[i].0].push(i);
    }

    let mut min_wbl = usize::MAX;
    for wbli in wbl.iter() {
        min_wbl = min_wbl.min(wbli.len());
    }

    let mut hy = vec![BinaryHeap::new(); min_wbl];
    for x in 0..w {
        for (c, i) in wbl[x].iter().enumerate() {
            if c < min_wbl {
                hy[c].push(xy[*i].1);
            }
        }
    }

    let mut blv = vec![-1; n];
    for x in 0..w {
        for (c, i) in wbl[x].iter().enumerate() {
            if c < min_wbl {
                blv[*i] = hy[c].peek().copied().unwrap() as isize;
            }
        }
    }
    debug!(blv);

    //

    let q: usize = tokens.next();
    for _ in 0..q {
        let t: usize = tokens.next();
        let a: usize = tokens.next();
        let t = t;
        let a = a - 1;
        debug!(a, t, blv[a]);

        if blv[a] < 0 || blv[a] >= t as isize {
            println!("Yes");
        }
        else {
            println!("No");
        }
    }
}
