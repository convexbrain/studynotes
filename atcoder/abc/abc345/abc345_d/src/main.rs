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

#[derive(Debug, Clone)]
struct ProdIter<I: Iterator>
{
    iters: Vec<I>,
}

#[derive(Debug, Clone)]
struct IterProd<'a, I: Iterator>
{
    iters_org: &'a[I],
    iters: Vec<I>,
    ret: Vec<I::Item>,
}

impl<I: Iterator + Clone> ProdIter<I>
{
    fn new() -> Self {
        ProdIter {
            iters: Vec::new(),
        }
    }

    fn push(&mut self, iter: I) {
        self.iters.push(iter);
    }

    fn iter(&self) -> IterProd<'_, I> {
        IterProd {
            iters_org: &self.iters,
            iters: self.iters.clone(),
            ret: Vec::new(),
        }
    }
}

impl<'a, I: Iterator + Clone> Iterator for IterProd<'a, I>
{
    type Item = &'a[I::Item];

    fn next(&mut self) -> Option<Self::Item> {
        if self.ret.len() == 0 {
            for pos in 0..self.iters.len() {
                if let Some(t) = self.iters[pos].next() {
                    self.ret.push(t);
                }
                else {
                    return None;
                }
            }

            Some(
                unsafe {
                    // self is borrowed as `&mut`, but this returns its contents as `&`.
                    // It violates `&mut` constraints.
                    std::mem::transmute(self.ret.as_slice())
                }
            )
        }
        else {
            for pos in (0..self.iters.len()).rev() {
                if let Some(t) = self.iters[pos].next() {
                    self.ret[pos] = t;
                    return Some(
                        unsafe {
                            // self is borrowed as `&mut`, but this returns its contents as `&`.
                            // It violates `&mut` constraints.
                            std::mem::transmute(self.ret.as_slice())
                        }
                    );
                }
                else {
                    self.iters[pos] = self.iters_org[pos].clone();
                    self.ret[pos] = self.iters[pos].next().unwrap();
                }
            }
            None
        }
    }
}

//#############################################################################

fn sub(map: &mut Vec<Vec<bool>>, h: usize, w: usize, x: usize, y: usize, a: usize, b: usize) -> bool {
    for ai in 0..a {
        for bi in 0..b {
            let xx = x + ai;
            let yy = y + bi;

            if yy >= h || xx >= w {
                return false;
            }
            else if map[yy][xx] {
                return false;
            }
            else {
                map[yy][xx] = true;
            }
        }
    }

    true
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let ab: Vec<(usize, usize)> = (0..n).map(|_| (tokens.next(), tokens.next())).collect();
    debug!(ab);

    for bit in 0..2_i32.pow(n as u32) {
        let mut v = Vec::new();
        
        let mut sz = 0;
        for i in 0..n {
            if bit & (1 << i) != 0 {
                v.push(i);
                sz += ab[i].0 * ab[i].1;
            }
        }

        if sz != h * w {
            continue;
        }

        let mut p = ProdIter::new();
        for _ in 0.. v.len() {
            p.push(0..(h * w * 2));
        }
        debug!(v);

        for vi in p.iter() {
            let mut map = vec![vec![false; w]; h];

            let mut check = true;
            for (i, vii) in vi.iter().enumerate() {
                let rev = (vii & 1) == 1;
                let x = (vii / 2) % w;
                let y = (vii / 2 / w) % h;
                let a = if rev {ab[v[i]].1} else {ab[v[i]].0};
                let b = if rev {ab[v[i]].0} else {ab[v[i]].1};

                if !sub(&mut map, h, w, x, y, a, b) {
                    check = false;
                    break;
                }
            }

            if check {
                let mut ok = true;
                for hi in 0..h {
                    for wi in 0..w {
                        if !map[hi][wi] {
                            ok = false;
                            break;
                        }
                    }
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
