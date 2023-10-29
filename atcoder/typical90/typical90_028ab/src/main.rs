use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::{collections::*, ops::*};
use std::{rc::*, cell::*, ops::Bound::*};

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

#[derive(Debug)]
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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let n: usize = token.next().unwrap().parse().unwrap();

    debug!(n);

    let mut intpl = Vec2D::from(vec![0_i32; 1001 * 1001], (1001, 1001));

    for _ in 0..n {
        let lx: usize = token.next().unwrap().parse().unwrap();
        let ly: usize = token.next().unwrap().parse().unwrap();
        let rx: usize = token.next().unwrap().parse().unwrap();
        let ry: usize = token.next().unwrap().parse().unwrap();

        debug!(lx, ly, rx, ry);

        intpl[(lx, ly)] += 1;
        intpl[(rx, ry)] += 1;
        intpl[(lx, ry)] -= 1;
        intpl[(rx, ly)] -= 1;
    }

    for x in 0..1001 {
        let mut int = 0;
        for y in 0..1001 {
            int += intpl[(x, y)];
            intpl[(x, y)] = int;
        }
    }

    let mut area = vec![0; n];

    for y in 0..1001 {
        let mut int = 0;
        for x in 0..1001 {
            int += intpl[(x, y)];

            if int > 0 {
                area[int as usize - 1] += 1;
            }
        }
    }

    debug!(area);

    for a in area {
        println!("{}", a);
    }
}
