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

    let s = tokens.next_bytes(); // Vec<u8>
    let mut a = vec![Vec::new(); 26];

    for (idx, sb) in s.iter().enumerate() {
        let ai = sb - b'A';
        let ai = ai as usize;
        a[ai].push(idx);
    }

    let mut cnt = 0;
    for i in 0..26 {
        let n = a[i].len();
        if n >= 2 {
            debug!(a[i]);

            for j in 1..n {
                let c = (a[i][j] - a[i][j - 1]) * j;
                let c = c * (n - j);
                debug!(c);
                cnt += c;
            }
            cnt -= n * (n - 1) / 2;
        }
    }
    println!("{cnt}");
}
