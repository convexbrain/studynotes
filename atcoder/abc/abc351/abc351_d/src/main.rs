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
struct Vec2D<T> {
    vec_row_major: Vec<T>,
    nrows: usize,
    ncols: usize,
}

impl<T> Vec2D<T> {
    fn from(vec_row_major: Vec<T>, (nrows, ncols): (usize, usize)) -> Self {
        assert!(vec_row_major.len() >= nrows * ncols);
        Self {vec_row_major, nrows, ncols}
    }

    fn release(self) -> Vec<T> {
        self.vec_row_major
    }
}

impl<T> Vec2D<T> {
    fn get(&self, row: isize, col: isize) -> Option<&T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }

    fn get_mut(&mut self, row: isize, col: isize) -> Option<&mut T> {
        if row < 0 || col < 0 || !(row < self.nrows as isize) || !(col < self.ncols as isize) {
            None
        }
        else {
            Some(&mut self.vec_row_major[row as usize * self.ncols + col as usize])
        }
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nrows);
        assert!(index.1 < self.ncols);
        &mut self.vec_row_major[index.0 * self.ncols + index.1]
    }
}

//#############################################################################

fn sub1(m: &mut Vec2D<i32>, r: usize, c: usize) {
    if m[(r, c)] != 0 {
        return;
    }

    for d in 0..4 {
        let (rr, cc) = match d {
            0 => {(r as isize - 1, c as isize)},
            1 => {(r as isize + 1, c as isize)},
            2 => {(r as isize, c as isize - 1)},
            3 => {(r as isize, c as isize + 1)},
            _ => {panic!()},
        };

        if let Some(mi) = m.get(rr, cc) {
            if *mi == -1 {
                m[(r, c)] = 1;
                return;
            }
        }
    }
}

fn sub2(m: &mut Vec2D<i32>, r: usize, c: usize) -> usize {
    if m[(r, c)] == -1 {
        return 0;
    }
    else if m[(r, c)] == 1 {
        return 1;
    }
    else if m[(r, c)] == 2 {
        return 0;
    }

    let mut vis = BTreeSet::new();
    let mut que = VecDeque::new();

    let mut s = 0;
    que.push_back((r, c));
    while let Some(pos) = que.pop_front() {
        if !vis.contains(&pos) {
            vis.insert(pos);

            if m[pos] == -1 {
                continue;
            }

            s += 1;

            if m[pos] == 0 {
                m[pos] = 2;
            }

            if m[pos] != 1 {
                for d in 0..4 {
                    let (rr, cc) = match d {
                        0 => {(pos.0 as isize - 1, pos.1 as isize)},
                        1 => {(pos.0 as isize + 1, pos.1 as isize)},
                        2 => {(pos.0 as isize, pos.1 as isize - 1)},
                        3 => {(pos.0 as isize, pos.1 as isize + 1)},
                        _ => {panic!()},
                    };

                    if let Some(_) = m.get(rr, cc) {
                        que.push_back((rr as usize, cc as usize));
                    }
                }
            }
        }
    }

    debug!(s);
    s
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();

    let mut m = Vec2D::from(vec![0; h * w], (h, w));

    for hi in 0..h {
        let s = tokens.next_string(); // String
        for (wi, c) in s.chars().enumerate() {
            if c == '#' {
                m[(hi, wi)] = -1;
            }
        }
    }
    debug!(m);

    for hi in 0..h {
        for wi in 0..w {
            sub1(&mut m, hi, wi);
        }
    }
    debug!(m);

    let mut max_s = 0;
    for hi in 0..h {
        for wi in 0..w {
            let s = sub2(&mut m, hi, wi);
            max_s = max_s.max(s);
        }
    }
    debug!(m);

    println!("{max_s}");
}
