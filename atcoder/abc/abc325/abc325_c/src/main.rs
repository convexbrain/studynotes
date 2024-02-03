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
    nr: usize,
    nc: usize,
}

impl<T> Vec2D<T> {
    fn from(vec_row_major: Vec<T>, (nr, nc): (usize, usize)) -> Self {
        assert!(vec_row_major.len() >= nr * nc);
        Self {vec_row_major, nr, nc}
    }

    fn release(self) -> Vec<T> {
        self.vec_row_major
    }
}

impl<T> Index<(usize, usize)> for Vec2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &self.vec_row_major[index.0 * self.nc + index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.nr);
        assert!(index.1 < self.nc);
        &mut self.vec_row_major[index.0 * self.nc + index.1]
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let h: usize = token.next().unwrap().parse().unwrap();
    let w: usize = token.next().unwrap().parse().unwrap();

    debug!(h, w);

    let mut v2 = Vec2D::from(vec![0; h * w], (h, w));

    let mut num_v = 0;
    let mut dup = BTreeSet::new();

    for hi in 0..h {
        let s = token.next().unwrap(); // &str

        debug!(s);

        for (idx, val) in s.bytes().enumerate() {
            let wi = idx;

            if val == b'#' {
                let mut nei = Vec::new();

                if wi > 0 && v2[(hi, wi - 1)] > 0 {
                    let vv = v2[(hi, wi - 1)];
                    if !nei.contains(&vv) {
                        nei.push(vv);
                    }
                }
                if hi > 0 {
                    for ww in (wi.max(1) - 1)..=((wi + 1).min(w - 1)) {
                        if v2[(hi - 1, ww)] > 0 {
                            let vv = v2[(hi - 1, ww)];
                            if !nei.contains(&vv) {
                                nei.push(vv);
                            }
                        }
                    }
                }

                debug!(nei);

                if nei.len() > 0 {
                    for i in 0..nei.len() {
                        for j in (i + 1)..nei.len() {
                            dup.insert((nei[i].min(nei[j]), nei[i].max(nei[j])));
                        }
                    }
                    debug!(dup);

                    v2[(hi, wi)] = nei[0];
                }
                else {
                    // new
                    num_v += 1;
                    v2[(hi, wi)] = num_v;
                }
            }
        }
    }

    debug!(v2);
    debug!(dup);

    let mut dups = vec![1; num_v];

    for d in dup.iter() {
        dups[d.1 - 1] = 0;
    }
    
    let dupsum: usize = dups.iter().sum();
    debug!(dups, dupsum);
    
    println!("{}", dupsum);
}
