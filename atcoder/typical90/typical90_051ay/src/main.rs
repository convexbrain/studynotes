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
        (0..n).map(|_| self.0.next().unwrap().parse().unwrap()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.0.next().unwrap().parse().unwrap())).collect()
    }
}

#[derive(Debug, Clone)]
struct CombIter
{
    n: usize,
    k: usize,
}

#[derive(Debug, Clone)]
struct IterComb<'a>
{
    comb: &'a CombIter,
    n_c_k: Vec<usize>,
    first: bool,
    end: bool,
}

impl CombIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        CombIter {n, k}
    }

    fn iter(&self) -> IterComb {
        let n_c_k = (0..self.k).collect();
        IterComb {
            comb: &self,
            n_c_k,
            first: true, end: false
        }
    }
}

impl<'a> Iterator for IterComb<'a>
{
    type Item = &'a[usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
            if self.comb.k == 0 {
                self.end = true;
            }
        }
        else if self.end {
            return None;
        }
        else {
            let n = self.comb.n;
            let k = self.comb.k;

            for pos in (0..k).rev() {
                let c = self.n_c_k[pos] + 1;
                if c < n - k + 1 + pos {
                    for i in pos..k {
                        self.n_c_k[i] = c + i - pos;
                    }
                    break;
                }
                else {
                    if pos == 0 {
                        self.end = true;
                        return None;
                    }
                }
            }
        }
    
        Some(
            unsafe {
                // self is borrowed as `&mut`, but this returns its contents as `&`.
                // It violates `&mut` constraints.
                std::mem::transmute(self.n_c_k.as_slice())
            }
        )
    }
}

//#############################################################################

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let k: usize = tokens.next();
    let p: u64 = tokens.next();
    debug!(n, k, p);

    let a0: Vec<u64> = tokens.collect(n / 2);
    let a1: Vec<u64> = tokens.collect(n - n / 2);
    debug!(a0, a1);

    let mut c0 = vec![Vec::new(); k + 1];
    let mut c1 = vec![Vec::new(); k + 1];
    for i in 0..=k {
        if i <= a0.len() {
            for c in CombIter::new(a0.len(), i).iter() {
                let sum = c.iter().fold(0, |acc, x| acc + a0[*x]);
                if sum <= p {
                    c0[i].push(sum);
                }
            }
            c0[i].sort();
        }
        if i <= a1.len() {
            for c in CombIter::new(a1.len(), i).iter() {
                let sum = c.iter().fold(0, |acc, x| acc + a1[*x]);
                if sum <= p {
                    c1[i].push(sum);
                }
            }
            c1[i].sort();
        }
    }
    debug!(c0, c1);

    let mut ans = 0;
    for i in 0..=k {
        for c in c0[i].iter() {
            debug!(i, c);
            let j = k - i;

            let r = p - c;
            let idx = match c1[j].binary_search(&r) {
                Ok(i) => {
                    let mut i = i;
                    while c1[j][i] == r {
                        i += 1;
                        if i == c1[j].len() {
                            break;
                        }
                    }
                    i
                },
                Err(i) => i,
            };
            debug!(idx);

            ans += idx;
        }
    }

    println!("{ans}");
}
