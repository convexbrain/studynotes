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

fn sub(map: &Vec<Vec<u64>>, x: u64, die: &mut BTreeSet<(usize, usize)>, nei: &mut BTreeSet<(u64, usize, usize)>, i: usize, j: usize, score: u64, ans: &mut u64) {
    let h = map.len();
    let w = map[0].len();

    let s = map[i][j];
    let score = score + s;
    *ans = score.max(*ans);

    die.insert((i, j));
    nei.remove(&(map[i][j], i, j));

    let mut v = Vec::new();
    if i > 0 && !die.contains(&(i - 1, j)) {
        v.push((i - 1, j));
    }
    if i < h - 1 && !die.contains(&(i + 1, j)) {
        v.push((i + 1, j));
    }
    if j > 0 && !die.contains(&(i, j - 1)) {
        v.push((i, j - 1));
    }
    if j < w - 1 && !die.contains(&(i, j + 1)) {
        v.push((i, j + 1));
    }
    let v = v;

    for vi in v.iter() {
        nei.insert((map[vi.0][vi.1], vi.0, vi.1));
    }

    let mut l = Vec::new();
    for ni in nei.iter() {
        if ni.0 < (score + x - 1) / x {
            l.push((ni.1, ni.2));
        }
        else {
            break;
        }
    }
    for li in l.iter() {
        sub(map, x, die, nei, li.0, li.1, score, ans);
    }

    for vi in v.iter() {
        nei.remove(&(map[vi.0][vi.1], vi.0, vi.1));
    }
    die.remove(&(i, j));
    nei.insert((map[i][j], i, j));
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let x: u64 = tokens.next();
    let p: usize = tokens.next();
    let q: usize = tokens.next();
    let mut map = Vec::new();
    for i in 0..h {
        let v: Vec<u64> = tokens.collect(w);
        map.push(v);
    }
    let map = map;

    let mut die = BTreeSet::new();
    let mut nei = BTreeSet::new();
    let mut ans = 0;
    sub(&map, x, &mut die, &mut nei, p - 1, q - 1, 0, &mut ans);
    println!("{ans}");
}
