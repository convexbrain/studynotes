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

    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for _ in 0..m {
        let xx: usize = tokens.next();
        let yy: usize = tokens.next();
        let zz: u32 = tokens.next();
        x.push(xx - 1);
        y.push(yy - 1);
        z.push(zz);
    }

    let mut a = vec![0_u32; n];

    for b in (0..32).rev() {
        let mut g = vec![(BTreeSet::new(), None); n];
        for i in 0..m {
            if z[i] & (1 << b) == 0 {
                g[x[i]].0.insert((y[i], false));
                g[y[i]].0.insert((x[i], false));
            }
            else {
                g[x[i]].0.insert((y[i], true));
                g[y[i]].0.insert((x[i], true));
            }
        }

        let mut u = BTreeSet::from_iter(0..n);
        while let Some(s) = u.first() {
            let mut q = VecDeque::new();
            q.push_back((*s, 0));
            while let Some(node) = q.pop_back() {
                if let Some(p) = g[node.0].1 {
                    if p != node.1 {
                        println!("-1");
                        return;
                    }
                }
                else {
                    g[node.0].1 = Some(node.1);
                }

                if u.contains(&node.0) {
                    u.remove(&node.0);
                    for e in g[node.0].0.iter() {
                        q.push_back((e.0, if e.1 {1 - node.1} else {node.1}));
                    }
                }
            }
        }

        for j in 0..n {
            if g[j].1.unwrap() == 1 {
                a[j] |= 1 << b;
            }
        }
    }

    for ai in a.iter() {
        print!("{} ", ai);
    }
    println!();
}
