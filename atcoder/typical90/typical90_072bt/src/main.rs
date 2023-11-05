use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::*,
    rc::*, cell::*, ops::Bound::*,
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

fn recursion(map: &mut Vec2D<i32>, max_len: &mut i32, r: isize, c: isize, mark: i32) {
    if let Some(tcs) = map.get(r, c) {
        match tcs {
            0 => {
                map[(r as usize, c as usize)] = mark;

                recursion(map, max_len, r - 1, c, mark + 1);
                recursion(map, max_len, r + 1, c, mark + 1);
                recursion(map, max_len, r, c - 1, mark + 1);
                recursion(map, max_len, r, c + 1, mark + 1);

                map[(r as usize, c as usize)] = 0;
            },
            1 => {
                let len = mark - 1;
                if len > 2 {
                    *max_len = (*max_len).max(len);
                }
            },
            _ => {},
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let h: usize = token.next().unwrap().parse().unwrap();
    let w: usize = token.next().unwrap().parse().unwrap();

    debug!(h, w);

    let mut cs = Vec2D::from(vec![0; h * w], (h, w));
    for r in 0..h {
        for (c, val) in token.next().unwrap().bytes().enumerate() {
            cs[(r, c)] = if val == b'#' {-1} else {0};
        }
    }

    debug!(cs);

    let mut max_len = -1;
    for r in 0..h {
        for c in 0..w {
            recursion(&mut cs, &mut max_len, r as isize, c as isize, 1);
        }
    }

    println!("{max_len}");
}
