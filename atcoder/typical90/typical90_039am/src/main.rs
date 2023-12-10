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
        (0..n).map(|_| self.0.next().unwrap().parse().unwrap()).collect()
    }
    fn collect_index<T, C>(&mut self, n: usize) -> C
    where T: std::str::FromStr, T::Err: std::fmt::Debug, C: FromIterator<(usize, T)> {
        (0..n).map(|i| (i, self.0.next().unwrap().parse().unwrap())).collect()
    }
}

//#############################################################################

fn dfs(nch: &mut[usize], g: &[HashSet<usize>], node: usize, pnode: Option<usize>) {
    nch[node] = 1;

    for &i in g[node].iter() {
        if let Some(pn) = pnode {
            if pn == i {
                continue;
            }
        }

        dfs(nch, g, i, Some(node));
        nch[node] += nch[i];
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    debug!(n);

    let mut e = Vec::new();
    let mut g = vec![HashSet::new(); n];

    for _ in 0..(n - 1) {
        let a: usize = tokens.next();
        let b: usize = tokens.next();
        debug!(a, b);

        let a = a - 1;
        let b = b - 1;
        e.push((a, b));

        g[a].insert(b);
        g[b].insert(a);
    }
    debug!(e);
    debug!(g);

    let mut nch = vec![0; n];
    dfs(&mut nch, &g, 0, None);
    debug!(nch);

    let mut ans = 0;
    let mut que = VecDeque::new();
    let mut vis = HashSet::new();
    que.push_back(0);
    while let Some(node) = que.pop_front() {
        vis.insert(node);

        for &i in g[node].iter() {
            if !vis.contains(&i) {
                let c = nch[i] * (n - nch[i]);
                ans += c;
                debug!(c, ans);

                que.push_back(i);
            }
        }
    }

    println!("{ans}");
}
