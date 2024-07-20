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

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let y: u32 = tokens.next();
    let v: Vec<u32> = tokens.collect(h * w);
    let mut v2 = Vec2D::from(v, (h, w));
    debug!(v2);

    let mut border = BinaryHeap::new();
    let mut not_inter = BTreeSet::new();
    for c in 0..w {
        border.push((Reverse(v2[(0, c)]), (0, c)));
        not_inter.insert((0, c));
        if h > 1 {
            border.push((Reverse(v2[(h - 1, c)]), (h - 1, c)));
            not_inter.insert((h - 1, c));
        }
    }
    for r in 1..(h - 1) {
        border.push((Reverse(v2[(r, 0)]), (r, 0)));
        not_inter.insert((r, 0));
        if w > 1 {
            border.push((Reverse(v2[(r, w - 1)]), (r, w - 1)));
            not_inter.insert((r, w - 1));
        }
    }

    let mut land = h * w;

    for year in 1..=y {
        loop {
            let peek = border.peek().clone();

            if peek.is_some() {
                let att = peek.unwrap().0.0;
                let pos = peek.unwrap().1;
                
                if att <= year {
                    border.pop();
                    land -= 1;

                    for i in 0..4 {
                        let q = match i {
                            0 => { if pos.0 > 0     {Some((pos.0 - 1, pos.1    ))} else {None} },
                            1 => { if pos.0 < h - 1 {Some((pos.0 + 1, pos.1    ))} else {None} },
                            2 => { if pos.1 > 0     {Some((pos.0    , pos.1 - 1))} else {None} },
                            3 => { if pos.1 < w - 1 {Some((pos.0    , pos.1 + 1))} else {None} },
                            _ => {panic!()},
                        };
                        if let Some(qq) = q {
                            if !not_inter.contains(&qq) {
                                not_inter.insert(qq);
                                border.push((Reverse(v2[qq]), qq));
                            }
                        }
                    }

                    continue;
                }
            }

            break;
        }
        println!("{land}");
    }
}
