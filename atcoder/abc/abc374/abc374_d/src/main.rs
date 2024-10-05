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
struct PermIter
{
    n: usize,
    k: usize,
}

#[derive(Debug, Clone)]
struct IterPerm<'a>
{
    perm: &'a PermIter,
    n_p_k: Vec<usize>,
    free: BTreeSet<usize>,
    first: bool,
    end: bool,
}

impl PermIter
{
    fn new(n: usize, k: usize) -> Self {
        assert!(n >= k);
        PermIter {n, k}
    }

    fn iter(&self) -> IterPerm {
        let n_p_k = (0..self.k).collect();
        let free = (self.k..self.n).collect();
        IterPerm {
            perm: &self,
            n_p_k, free,
            first: true, end: false,
        }
    }
}

impl<'a> Iterator for IterPerm<'a>
{
    type Item = &'a[usize];

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false;
        }
        else if self.end {
            return None;
        }
        else {
            let k = self.perm.k;

            for pos in (0..k).rev() {
                let c = self.n_p_k[pos];
                self.free.insert(c);
                if let Some(&nc) = self.free.range((c + 1)..).next() {
                    self.n_p_k[pos] = nc;
                    self.free.remove(&nc);
                    for i in (pos + 1)..k {
                        self.n_p_k[i] = self.free.pop_first().unwrap();
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
                std::mem::transmute(self.n_p_k.as_slice())
            }
        )
    }
}

//#############################################################################

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let s: f64 = tokens.next();
    let t: f64 = tokens.next();
    let abcd: Vec<(f64, f64, f64, f64)> = (0..n).map(|_| (tokens.next(), tokens.next(), tokens.next(), tokens.next())).collect();
    debug!(abcd);

    let time_t = abcd.iter().fold(0_f64, |acc, x| {
        acc + (x.0 - x.2).hypot(x.1 - x.3) / t
    });
    debug!(time_t);

    let mut time_s_min = f64::MAX;
    for p in PermIter::new(n, n).iter() {
        for b in 0_usize..(1 << n) {
            let mut time_s = 0_f64;
            for i in 0..n {
                let ab = if i == 0 {
                    (0_f64, 0_f64)
                }
                else if b & (1 << (i - 1)) != 0 {
                    (abcd[p[i - 1]].2, abcd[p[i - 1]].3)
                }
                else {
                    (abcd[p[i - 1]].0, abcd[p[i - 1]].1)
                };
                let cd = if b & (1 << i) != 0 {
                    (abcd[p[i]].0, abcd[p[i]].1)
                }
                else {
                    (abcd[p[i]].2, abcd[p[i]].3)
                };
                time_s += (ab.0 - cd.0).hypot(ab.1 - cd.1);
            }
            time_s /= s;
            debug!(time_s);
            time_s_min = time_s_min.min(time_s);
        }
    }

    println!("{}", time_t + time_s_min);
}
