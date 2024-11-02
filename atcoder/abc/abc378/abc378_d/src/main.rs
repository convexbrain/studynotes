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

fn sub(m: &Vec<Vec<bool>>, (h, w): (isize, isize), vis: &mut BTreeSet<(usize, usize)>, (i, j): (isize, isize), c: usize) -> usize {
    if i < 0 || j < 0 || i >= h || j >= w {
        return 0;
    }

    let ii = i as usize;
    let jj = j as usize;
    if m[ii][jj] {
        return 0;
    }

    if vis.contains(&(ii, jj)) {
        return 0;
    }

    if c == 0 {
        return 1;
    }

    vis.insert((ii, jj));
    let cnt =       sub(m, (h, w), vis, (i - 1, j    ), c - 1);
    let cnt = cnt + sub(m, (h, w), vis, (i + 1, j    ), c - 1);
    let cnt = cnt + sub(m, (h, w), vis, (i    , j - 1), c - 1);
    let cnt = cnt + sub(m, (h, w), vis, (i    , j + 1), c - 1);
    vis.remove(&(ii, jj));

    return cnt;
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let k: usize = tokens.next();

    let mut m = vec![vec![false; w]; h];
    for i in 0..h {
        let s = tokens.next_string(); // String
        for (j, c) in s.char_indices() {
            m[i][j] = c == '#';
        }
    }

    let mut cnt = 0;
    for i in 0..h {
        for j in 0..w {
            let mut vis = BTreeSet::new();
            cnt += sub(&m, (h as isize, w as isize), &mut vis, (i as isize, j as isize), k);
        }
    }
    println!("{cnt}");
}
