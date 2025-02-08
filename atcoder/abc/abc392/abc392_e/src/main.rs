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
    let m: usize = tokens.next();
    let mut ab = Vec::new();
    for _ in 0..m {
        let a: usize = tokens.next();
        let b: usize = tokens.next();
        ab.push((a - 1, b - 1));
    }
    debug!(ab);

    let mut g = vec![BTreeSet::new(); n];
    for abe in ab.iter().enumerate() {
        let a = abe.1.0;
        let b = abe.1.1;
        g[a].insert(abe.0);
        g[b].insert(abe.0);
    }
    let mut gg = vec![0; n];

    let mut group = Vec::new();
    let mut unvis: BTreeSet<usize> = (0..n).collect();
    let mut usedc = BTreeSet::new();
    let mut fc = BTreeSet::new();
    while !unvis.is_empty() {
        let mut gp = BTreeSet::new();

        let mut que = VecDeque::new();
        que.push_back((*unvis.first().unwrap(), usize::MAX));
        while let Some(sc) = que.pop_front() {
            if unvis.contains(&sc.0) {
                unvis.remove(&sc.0);
                gp.insert(sc.0);
                gg[sc.0] = group.len();
                usedc.insert(sc.1);

                for &c in g[sc.0].iter() {
                    if sc.0 != ab[c].0 {
                        que.push_back((ab[c].0, c));
                    }
                    else if sc.0 != ab[c].1 {
                        que.push_back((ab[c].1, c));
                    }
                    else {
                        fc.insert((c, group.len()));
                    }
                }
            }
            else {
                if !usedc.contains(&sc.1) {
                    fc.insert((sc.1, group.len()));
                }
            }
        }

        group.push(gp);
    }
    debug!(group, fc);

    let mut gps: BTreeSet<usize> = (1..(group.len())).collect();
    println!("{}", group.len() - 1);
    if !gps.is_empty() {
        for fci in fc.iter() {
            if !gps.contains(&fci.1) {
                let gp = gps.pop_first().unwrap();
                let s = group[gp].first().unwrap();
                println!("{} {} {}", fci.0 + 1, ab[fci.0].0 + 1, s + 1);
            }
            else {
                gps.remove(&fci.1);
                let s = group[0].first().unwrap();
                println!("{} {} {}", fci.0 + 1, ab[fci.0].0 + 1, s + 1);
            }

            if gps.is_empty() {
                return;
            }
        }
    }
}
