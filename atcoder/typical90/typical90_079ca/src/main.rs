use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;
use std::rc::*;
use std::ops::*;
use std::cmp::*;

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
    vec_row_wise: Vec<T>,
    nr: usize,
    nc: usize,
}

impl<T> Vec2D<T> {
    fn from(vec_row_wise: Vec<T>, (nr, nc): (usize, usize)) -> Self {
        assert!(vec_row_wise.len() >= nr * nc);
        Self {vec_row_wise, nr, nc}
    }

    fn release(self) -> Vec<T> {
        self.vec_row_wise
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &self.vec_row_wise[index.0 * self.nc + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &mut self.vec_row_wise[index.0 * self.nc + index.1]
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let h: usize = token.next().unwrap().parse().unwrap();
    let w: usize = token.next().unwrap().parse().unwrap();

    debug!(h, w);

    let a: Vec<i32> = (0..(h*w)).map(|_| token.next().unwrap().parse().unwrap()).collect();
    let b: Vec<i32> = (0..(h*w)).map(|_| token.next().unwrap().parse().unwrap()).collect();

    debug!(a, b);

    let b_a: Vec<i32> = a.iter().zip(b.iter())
                         .map(|x| x.1 - x.0).collect();
    let mut b_a = Vec2D::from(b_a, (h, w));

    debug!(b_a);

    let mut cnt = 0_u64;
    for r in 0..(h - 1) {
        for c in 0..(w - 1) {
            let v = b_a[(r, c)];
            b_a[(r, c)] = 0;
            b_a[(r + 1, c)] -= v;
            b_a[(r, c + 1)] -= v;
            b_a[(r + 1, c + 1)] -= v;
            cnt += v.abs() as u64;
        }
        debug!(b_a);
    }

    let b_a = b_a.release();

    if b_a.iter().any(|&x| x != 0) {
        println!("No");
        return;
    }

    println!("Yes\n{}", cnt);
}
