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
    let mut d = Vec::new();
    for _ in 0..n {
        let k: usize = tokens.next();
        let mut am = BTreeMap::new();
        for _ in 0..k {
            let a: u32 = tokens.next();
            am.entry(a).and_modify(|e| *e += 1).or_insert(1_usize);
        }
        d.push((k, am));
    }
    debug!(d);

    let mut ans = 0_f64;
    for i in 0..n {
        for j in (i + 1)..n {
            let (am0, am1) = if d[i].1.len() <  d[j].1.len() {
                (&d[i].1, &d[j].1)
            }
            else {
                (&d[j].1, &d[i].1)
            };

            let mut a = 0;
            for am0i in am0.iter() {
                if let Some(am1g) = am1.get(am0i.0) {
                    a += am0i.1 * am1g;
                }
            }
            let af = a as f64 / d[i].0 as f64 / d[j].0 as f64;
            debug!(a, af);

            ans = ans.max(af);
        }
    }
    println!("{ans}");
}
