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

fn sub(a: &Vec<Vec<i64>>, n: usize, p: i64, x: i64) -> usize {
    let mut d = vec![vec![0_i64; n]; n];

    for r in 0..n {
        for c in 0..n {
            d[r][c] = if a[r][c] != -1 {a[r][c]} else {x};
        }
    }

    for k in 0..n {
        for r in 0..n {
            for c in 0..n {
                d[r][c] = d[r][c].min(d[r][k] + d[k][c]);
            }
        }
    }

    let mut cnt = 0;
    for r in 0..n {
        for c in (r + 1)..n {
            if d[r][c] <= p {
                cnt += 1;
            }
        }
    }

    cnt
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let p: i64 = tokens.next();
    let k: usize = tokens.next();
    debug!(n, p, k);

    let mut a = vec![vec![0_i64; n]; n];
    for r in 0..n {
        for c in 0..n {
            a[r][c] = tokens.next();
        }
    }
    debug!(a);

    let mut ll = 1;
    let mut lr = p + 1;
    loop {
        //debug!(ll, lr);
        let m = (ll + lr) / 2;
        let kk = sub(&a, n, p, m);
        //debug!(m, kk);
        if kk <= k {
            lr = m;
        }
        else {
            ll = m + 1;
        }

        if ll >= lr {
            break;
        }
    }
    debug!(lr);

    let mut rl = 1;
    let mut rr = p + 1;
    loop {
        //debug!(rl, rr);
        let m = (rl + rr) / 2;
        let kk = sub(&a, n, p, m);
        //debug!(m, kk);
        if kk < k {
            rr = m;
        }
        else {
            rl = m + 1;
        }

        if rl >= rr {
            break;
        }
    }
    debug!(rr);

    let kp1 = sub(&a, n, p, p + 1);

    if kp1 == k {
        println!("Infinity");
    }
    else if kp1 > k {
        println!("0");
    }
    else {
        println!("{}", rr - lr);
    }
}
