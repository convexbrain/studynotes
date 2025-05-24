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
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
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
}

//#############################################################################

fn sub(m: &Vec<Vec<u64>>, d: &mut Vec<Vec<bool>>, h: usize, w: usize, i: usize, j: usize, score: u64, max_score: &mut u64) {
    if d[i][j] {
        if i + 1 < h {
            sub(m, d, h, w, i + 1, j, score, max_score);
        }
        else if j + 1 < w {
            sub(m, d, h, w, 0, j + 1, score, max_score);
        }
        else {
            if score > *max_score {
                debug!(score);
            }
            *max_score = score.max(*max_score);
        }
    }
    else {
        d[i][j] = true;

        if i + 1 < h && !d[i + 1][j] {
            d[i + 1][j] = true;
            sub(m, d, h, w, i + 1, j, score, max_score);
            d[i + 1][j] = false;
        }

        if j + 1 < w {
            d[i][j + 1] = true;
            if i + 1 < h {
                sub(m, d, h, w, i + 1, j, score, max_score);
            }
            else {
                sub(m, d, h, w, 0, j + 1, score, max_score);
            }
            d[i][j + 1] = false;
        }

        d[i][j] = false;

        let s = score ^ m[i][j];
        if i + 1 < h {
            sub(m, d, h, w, i + 1, j, s, max_score);
        }
        else if j + 1 < w {
            sub(m, d, h, w, 0, j + 1, s, max_score);
        }
        else {
            if s > *max_score {
                debug!(s);
            }
            *max_score = s.max(*max_score);
        }
    }

}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let mut m = Vec::new();
    for _ in 0..h {
        let a: Vec<u64> = tokens.collect(w);
        m.push(a);
    }
    debug!(m);

    let mut d = vec![vec![false; w]; h];
    let mut max_score = 0;
    sub(&m, &mut d, h, w, 0, 0, 0, &mut max_score);
    println!("{max_score}");
}
