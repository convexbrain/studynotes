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
        //std::io::stdin().read_to_string(buf).unwrap();
        std::io::stdin().read_line(buf).unwrap();
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

    let mut g = vec![BTreeSet::new(); n];
    for _ in 0..(n - 1) {
        let mut tokens_buf = String::new();
        let mut tokens = Tokens::new(&mut tokens_buf);
    
        let u: usize = tokens.next();
        let v: usize = tokens.next();
        let u = u - 1;
        let v = v - 1;
        g[u].insert(v);
        g[v].insert(u);
    }

    let mut pair = BTreeSet::new();
    for start in 0..n {
        let mut vis = BTreeSet::new();
        let mut que = VecDeque::new();
        que.push_back((start, 0));
        while let Some(q) = que.pop_front() {
            if !vis.contains(&q.0) {
                vis.insert(q.0);

                if q.1 > 1 && q.1 % 2 == 1 {
                    let p = if start < q.0 {(start, q.0)} else {(q.0, start)};
                    pair.insert(p);
                }

                for next in g[q.0].iter() {
                    que.push_back((*next, q.1 + 1));
                }
            }
        }
    }
    debug!(pair);

    let first = pair.len() % 2 == 1;
    if first {
        println!("First");
        let p = pair.pop_first().unwrap();
        println!("{} {}", p.0 + 1, p.1 + 1);
    }
    else {
        println!("Second");
    }
    std::io::stdout().flush().unwrap();

    loop {
        let mut tokens_buf = String::new();
        let mut tokens = Tokens::new(&mut tokens_buf);
    
        let i: isize = tokens.next();
        let j: isize = tokens.next();
        if i > 0 && j > 0 {
            let i = i as usize - 1;
            let j = j as usize - 1;
            let ij = if i < j {(i, j)} else {(j, i)};
            pair.remove(&ij);

            let p = pair.pop_first().unwrap();
            println!("{} {}", p.0 + 1, p.1 + 1);
            std::io::stdout().flush().unwrap();
        }
        else {
            break;
        }
    }
}
