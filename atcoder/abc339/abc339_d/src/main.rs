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

    let mut m = vec![vec![false; n]; n];

    let mut p = (None, None);

    for hi in 0..n {
        let s = tokens.next_string(); // String
        for (wi, c) in s.chars().enumerate() {
            match c {
                'P' => {
                    if let Some(_) = p.0 {
                        p.1 = Some((hi, wi));
                    }
                    else {
                        p.0 = Some((hi, wi));
                    }
                },
                '.' => {},
                '#' => {
                    m[hi][wi] = true;
                },
                _ => {panic!()},
            }
        }
    }
    let p = (p.0.unwrap(), p.1.unwrap());

    debug!(m, p);

    let mut vis = HashSet::new();
    let mut que = VecDeque::new();

    que.push_front((p, 0));
    let mut min_step = u64::MAX;

    while let Some((ps, step)) = que.pop_front() {
        if !vis.contains(&ps) {
            vis.insert(ps);

            let step = step + 1;
            let p0 = ps.0;
            let p1 = ps.1;
            for d in 0..4 {
                let n_p0;
                let n_p1;
                match d {
                    0 => {
                        n_p0 = (if p0.0 > 0 {p0.0 - 1} else {p0.0}, p0.1);
                        n_p1 = (if p1.0 > 0 {p1.0 - 1} else {p1.0}, p1.1);
                    },
                    1 => {
                        n_p0 = (if p0.0 < n - 1 {p0.0 + 1} else {p0.0}, p0.1);
                        n_p1 = (if p1.0 < n - 1 {p1.0 + 1} else {p1.0}, p1.1);
                    },
                    2 => {
                        n_p0 = (p0.0, if p0.1 > 0 {p0.1 - 1} else {p0.1});
                        n_p1 = (p1.0, if p1.1 > 0 {p1.1 - 1} else {p1.1});
                    },
                    3 => {
                        n_p0 = (p0.0, if p0.1 < n - 1 {p0.1 + 1} else {p0.1});
                        n_p1 = (p1.0, if p1.1 < n - 1 {p1.1 + 1} else {p1.1});
                    },
                    _=> {panic!();},
                }

                let n_p0 = if m[n_p0.0][n_p0.1] {p0} else {n_p0};
                let n_p1 = if m[n_p1.0][n_p1.1] {p1} else {n_p1};

                if n_p0 == n_p1 {
                    min_step = min_step.min(step);
                }
                else {
                    let n_ps = (n_p0, n_p1);
                    if ps != n_ps && min_step > step {
                        que.push_back((n_ps, step));
                    }
                }
            }
        }
    }

    if min_step == u64::MAX {
        println!("-1");
    }
    else {
        println!("{min_step}");
    }
}
