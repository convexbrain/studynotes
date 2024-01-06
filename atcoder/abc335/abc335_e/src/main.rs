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

fn sub(
    graph: &[(u64, HashSet<usize>)],
    vis: &mut HashSet<usize>,
    smap: &mut HashMap<u64, usize>,
    score_max: &mut usize,
    score: usize,
    node: usize,
) {
    vis.insert(node);

    let score = if smap.contains_key(&graph[node].0) {
        smap.insert(graph[node].0, smap[&graph[node].0] + 1);
        score
    }
    else {
        smap.insert(graph[node].0, 1);
        score + 1
    };

    debug!(node);

    if node == graph.len() - 1 {
        *score_max = score.max(*score_max);
        debug!(score_max);
    }
    else {
        for &nn in graph[node].1.iter() {
            if !vis.contains(&nn) {
                if graph[node].0 <= graph[nn].0 {
                    sub(graph, vis, smap, score_max, score, nn);
                }
            }
        }
    }

    vis.remove(&node);
    if smap[&graph[node].0] > 1 {
        smap.insert(graph[node].0, smap[&graph[node].0] - 1);
    }
    else {
        smap.remove(&graph[node].0);
    };
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let m: usize = tokens.next();

    let mut g = vec![(0, HashSet::new()); n];

    for i in 0..n {
        let a: u64 = tokens.next();

        g[i].0 = a;
    }
    for _ in 0..m {
        let u: usize = tokens.next();
        let v: usize = tokens.next();
        let u = u - 1;
        let v = v - 1;

        g[u].1.insert(v);
        g[v].1.insert(u);
    }
    debug!(g);

    let mut vis = HashSet::new();
    let mut smap = HashMap::new();
    let mut score_max = 0;
    sub(&mut g, &mut vis, &mut smap, &mut score_max, 0, 0);

    println!("{score_max}");
}
