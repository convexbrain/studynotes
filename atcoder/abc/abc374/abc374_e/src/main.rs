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
    let x: u32 = tokens.next();
    let apbq: Vec<(u32, u32, u32, u32)> = (0..n).map(|_| (tokens.next(), tokens.next(), tokens.next(), tokens.next())).collect();
    debug!(apbq);

    let mut w_sup = 0;
    for i in 0..n {
        let ws = apbq[i].0 * (x / apbq[i].1);
        let wt = apbq[i].2 * (x / apbq[i].3);
        w_sup = w_sup.max(ws.max(wt));
    }
    w_sup += 1;

    let mut l = 0;
    let mut r = w_sup;
    while l + 1 < r {
        let w = (l + r) / 2;
        let mut c_sum = 0;
        for i in 0..n {
            let s_s = w / apbq[i].0;
            let t_s = (w - apbq[i].0 * s_s  + apbq[i].2 - 1) / apbq[i].2;
            let cs = apbq[i].1 * s_s + apbq[i].3 * t_s;
            let cs = cs.min(apbq[i].1 * (s_s + 1));

            let t_t = w / apbq[i].2;
            let s_t = (w - apbq[i].2 * t_t  + apbq[i].0 - 1) / apbq[i].0;
            let ct = apbq[i].3 * t_t + apbq[i].1 * s_t;
            let ct = ct.min(apbq[i].3 * (t_t + 1));

            let c = cs.min(ct);
            c_sum += c;
        }
        if c_sum > x {
            r = w;
        }
        else {
            l = w;
        }
    }
    println!("{l}");
}
