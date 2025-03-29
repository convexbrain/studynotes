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
    let s = tokens.next_bytes(); // Vec<u8>
    let t = tokens.next_bytes(); // Vec<u8>

    let mut g = vec![(BTreeSet::new(), false); 26];
    for i in 0..n {
        let si = (s[i] - b'a') as usize;
        let ti = (t[i] - b'a') as usize;
        if si == ti {
            g[si].1 = true;
        }
        else {
            g[si].0.insert(ti);
            g[ti].0.insert(si);
        }
    }

    let mut ans0 = 0;
    let mut ans1 = 0;
    let mut unvis = BTreeSet::from_iter(0..26);
    while let Some(start) = unvis.first() {
        let mut q: VecDeque<(usize, Option<usize>)> = VecDeque::new();

        q.push_front((*start, None));
        while let Some(p) = q.pop_front() {
            if unvis.contains(&p.0) {
                debug!(p);
                if p.1.is_some() {
                    ans0 += 1;
                }
                unvis.remove(&p.0);

                for &np in g[p.0].0.iter() {
                    if g[np].1 {
                        println!("-1");
                        return;
                    }
                    else if let Some(pp) = p.1 {
                        if np != pp {
                            q.push_front((np, Some(p.0)));
                        }
                    }
                    else {
                        q.push_front((np, Some(p.0)));
                    }
                }
            }
            else {
                debug!(p);
                if p.1.is_some() {
                    ans1 += 1;
                }
            }
        }
    }
    debug!(ans0, ans1);
    println!("{}", ans0 + ans1);
}
