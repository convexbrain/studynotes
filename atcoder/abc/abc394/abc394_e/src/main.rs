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

fn sub(g: &[(BTreeMap<char, BTreeSet<usize>>, BTreeMap<char, BTreeSet<usize>>)], i: usize, j: usize) -> Option<usize> {
    let n = g.len();
    let mut q = VecDeque::new();
    q.push_front((i, j, 0));
    while let Some(qf) = q.pop_front() {
        if qf.0 == qf.1 {
            return Some(qf.2 * 2);
        }
        else if qf.2 < n * n {
            for x in g[qf.0].0.iter() {
                for ii in x.1.iter() {
                    if *ii == qf.1 {
                        return Some(qf.2 * 2 + 1);
                    }
                }
                if g[qf.1].1.contains_key(x.0) {
                    for ii in x.1.iter() {
                        for jj in g[qf.1].1[x.0].iter() {
                            q.push_back((*ii, *jj, qf.2 + 1));
                        }
                    }
                }
            }
        }
    }
    None
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let mut g: Vec<(BTreeMap<char, BTreeSet<usize>>, BTreeMap<char, BTreeSet<usize>>)> = vec![(BTreeMap::new(), BTreeMap::new()); n];
    for i in 0..n {
        let cs = tokens.next_string(); // String
        for (j, c) in cs.char_indices() {
            if c != '-' {
                g[i].0.entry(c).and_modify(|e| {e.insert(j);}).or_insert(BTreeSet::from([j]));
                g[j].1.entry(c).and_modify(|e| {e.insert(i);}).or_insert(BTreeSet::from([i]));
            }
        }
    }
    debug!(g);

    for i in 0..n {
        for j in 0..n {
            let ans = sub(&g, i, j);
            if let Some(a) = ans {
                print!("{} ", a);
            }
            else {
                print!("{} ", -1);
            }
        }
        println!();
    }
}
