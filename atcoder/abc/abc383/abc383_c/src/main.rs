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

fn sub(map: &Vec<Vec<char>>, h: usize, w: usize, d: usize, vis: &mut BTreeMap<(usize, usize), bool>, i: usize, j: usize, s: usize) -> bool {
    if vis.contains_key(&(i, j)) {
        vis[&(i, j)]
    }
    else {
        let res = match map[i][j] {
            '.' => {
                if s < d {
                    let mut found = false;
                    found |= if i > 0     {sub(map, h, w, d, vis, i - 1, j, s + 1)} else {false};
                    found |= if i < h - 1 {sub(map, h, w, d, vis, i + 1, j, s + 1)} else {false};
                    found |= if j > 0     {sub(map, h, w, d, vis, i, j - 1, s + 1)} else {false};
                    found |= if j < w - 1 {sub(map, h, w, d, vis, i, j + 1, s + 1)} else {false};
                    found
                }
                else {
                    false
                }
            },
            'H' => {
                true
            },
            _ => {
                false
            }
        };
        vis.insert((i, j), res);
        res
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let d: usize = tokens.next();

    let mut map = Vec::new();
    for i in 0..h {
        let s = tokens.next_string(); // String
        let v: Vec<char> = s.chars().collect();
        map.push(v);
    }
    debug!(map);

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let mut vis = BTreeMap::new();
            if sub(&map, h, w, d, &mut vis, i, j, 0) {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
