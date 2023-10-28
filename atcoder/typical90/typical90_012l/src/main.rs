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
    let q: usize = token.next().unwrap().parse().unwrap();

    debug!(h, w, q);

    let mut map = Vec2D::from(vec![0; h * w], (h, w));
    let mut dirty = false;
    let mut que = VecDeque::new();
    let mut vis = HashSet::new();

    for i in 1..=q {
        let qq: usize = token.next().unwrap().parse().unwrap();

        debug!(qq);

        match qq {
            1 => {
                let r: usize = token.next().unwrap().parse().unwrap();
                let c: usize = token.next().unwrap().parse().unwrap();
                let r = r - 1;
                let c = c - 1;

                map[(r, c)] = i;
                for d in [(-1_isize, 0), (1, 0), (0, -1), (0, 1)] {
                    let r1 = r as isize + d.0;
                    let c1 = c as isize + d.1;
                    if let Some(v) = map.get(r1, c1) {
                        if *v > 0 {
                            dirty = true;
                        }
                    }
                }

                debug!(r, c);
            },
            2 => {
                let ra: usize = token.next().unwrap().parse().unwrap();
                let ca: usize = token.next().unwrap().parse().unwrap();
                let rb: usize = token.next().unwrap().parse().unwrap();
                let cb: usize = token.next().unwrap().parse().unwrap();
                let ra = ra - 1;
                let ca = ca - 1;
                let rb = rb - 1;
                let cb = cb - 1;

                if map[(ra, ca)] > 0 && map[(rb, cb)] > 0 {
                    if map[(ra, ca)] == map[(rb, cb)] {
                        println!("Yes");
                    }
                    else if dirty {
                        vis.clear();
                        for r0 in 0..h {
                            for c0 in 0..w {
                                let m0 = map[(r0, c0)];
                                if m0 > 0 && !vis.contains(&(r0, c0)) {
                                    que.push_back((r0, c0));
                                    while let Some((r1, c1)) = que.pop_front() {
                                        map[(r1, c1)] = m0;
                                        vis.insert((r1, c1));
                    
                                        for d in [(-1_isize, 0), (1, 0), (0, -1), (0, 1)] {
                                            let r2 = r1 as isize + d.0;
                                            let c2 = c1 as isize + d.1;
                                            if let Some(v) = map.get(r2, c2) {
                                                if *v > 0 && !vis.contains(&(r2 as usize, c2 as usize)) {
                                                    que.push_front((r2 as usize, c2 as usize));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        dirty = false;
                        if map[(ra, ca)] == map[(rb, cb)] {
                            println!("Yes");
                        }
                        else {
                            println!("No");
                        }
                    } else {
                        println!("No");
                    }
                }
                else {
                    println!("No");
                }

                debug!(ra, ca, rb, cb);
            },
            _ => {panic!()},
        }
    }
}
