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

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let q: usize = tokens.next();
    debug!(n, q);

    let mut t1 = Vec::new();
    let mut set = BTreeSet::from_iter(0..n);
    let mut g0 = vec![None; n - 1];

    for _ in 0..q {
        let t: usize = tokens.next();
        let x: usize = tokens.next();
        let y: usize = tokens.next();
        let v: i64 = tokens.next();
        //debug!(t, x, y, v);

        let x = x - 1;
        let y = y - 1;

        match t {
            0 => {
                g0[x] = Some(v);
                set.remove(&x);
            },
            1 => {
                let (xx, yy) = if x < y {(x, y)} else {(y, x)};
                if set.range(xx..yy).count() == 0 {
                    t1.push(Some((x, y, v)));
                }
                else {
                    t1.push(None);
                }
            },
            _ => {
                panic!();
            }
        }
    }
    debug!(t1, g0);

    let mut val0 = vec![0; n];
    let mut v = 0;
    for i in 1..n {
        if let Some(vv) = g0[i - 1] {
            v = vv - v;
        }
        else {
            v = 0;
        }
        val0[i] = v;
    }
    debug!(val0);

    for ti in t1.iter() {
        if let Some((x, y, v)) = ti {
            let x = *x;
            let y = *y;
            let ans = if x.abs_diff(y) % 2 == 0 {
                val0[y] + v - val0[x]
            }
            else {
                val0[y] + val0[x] - v
            };
            println!("{ans}");
        }
        else {
            println!("Ambiguous");
        }
    }
}
