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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let h: usize = token.next().unwrap().parse().unwrap();
    let w: usize = token.next().unwrap().parse().unwrap();
    let rs: usize = token.next().unwrap().parse().unwrap();
    let cs: usize = token.next().unwrap().parse().unwrap();
    let rt: usize = token.next().unwrap().parse().unwrap();
    let ct: usize = token.next().unwrap().parse().unwrap();

    debug!(h, w, rs, cs, rt, ct);

    let rs = rs - 1;
    let cs = cs - 1;
    let rt = rt - 1;
    let ct = ct - 1;

    let mut v = Vec::new();
    for _ in 0..h {
        let s = token.next().unwrap(); // &str

        debug!(s);

        let mut s: Vec<Option<(u32, u32, u32, u32)>> = s.bytes().map(|b|
            if b == b'#' {None} else {Some((u32::MAX, u32::MAX, u32::MAX, u32::MAX))}
        ).collect();
        v.append(&mut s);
    }
    let mut v2 = Vec2D::from(v, (h, w));
    
    debug!(v2);

    let mut que = VecDeque::new();
    que.push_front((rs as isize, cs as isize, 0, 0b1111));

    while let Some((r, c, s, d)) = que.pop_front() {
        if let Some(val) = v2.get_mut(r, c) {
            if let Some(val) = val {
                if d & 0b1000 != 0 && s < val.0 {
                    val.0 = s;
                    que.push_front((r, c - 1, s, 0b1000));
                }
                if d & 0b1000 == 0 && s + 1 < val.0 {
                    val.0 = s + 1;
                    que.push_back((r, c - 1, s + 1, 0b1000));
                }

                if d & 0b0100 != 0 && s < val.1 {
                    val.1 = s;
                    que.push_front((r, c + 1, s, 0b0100));
                }
                if d & 0b0100 == 0 && s + 1 < val.1 {
                    val.1 = s + 1;
                    que.push_back((r, c + 1, s + 1, 0b0100));
                }

                if d & 0b0010 != 0 && s < val.2 {
                    val.2 = s;
                    que.push_front((r - 1, c, s, 0b0010));
                }
                if d & 0b0010 == 0 && s + 1 < val.2 {
                    val.2 = s + 1;
                    que.push_back((r - 1, c, s + 1, 0b0010));
                }

                if d & 0b0001 != 0 && s < val.3 {
                    val.3 = s;
                    que.push_front((r + 1, c, s, 0b0001));
                }
                if d & 0b0001 == 0 && s + 1 < val.3 {
                    val.3 = s + 1;
                    que.push_back((r + 1, c, s + 1, 0b0001));
                }
            }
        }
    }

    debug!(v2);

    let val = v2[(rt, ct)].unwrap();
    let ans = val.0.min(val.1);
    let ans = ans.min(val.2);
    let ans = ans.min(val.3);

    println!("{}", ans);
}
