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

    let mut g = vec![BTreeSet::new(); n];

    for _ in 0..m {
        let x: usize = tokens.next();
        let y: usize = tokens.next();
        let z: u32 = tokens.next();
        let x = x - 1;
        let y = y - 1;
        g[x].insert((y, z));
        g[y].insert((x, z));
    }

    let mut a = vec![0_u32; n];

    for b in 0..32 {
        let mut u = BTreeSet::from_iter(0..n);
        while let Some(s) = u.first() {
            let mut sf = BTreeSet::new();
            let mut st = BTreeSet::new();
    
            let mut q = VecDeque::new();
            q.push_back((*s, false));
            while let Some(p) = q.pop_back() {
                if p.1 && !sf.contains(&p.0) {
                    st.insert(p.0);
                }
                else if !p.1 && !st.contains(&p.0) {
                    sf.insert(p.0);
                }
                else {
                    println!("-1");
                    return;
                }

                if u.contains(&p.0) {
                    u.remove(&p.0);
                    for e in g[p.0].iter() {
                        let ft = if e.1 & (1 << b) == 0 {p.1} else {!p.1};
                        q.push_back((e.0, ft));
                    }
                }
            }

            let s1 = if sf.len() > st.len() {st} else {sf};
            for &j in s1.iter() {
                a[j] |= 1 << b;
            }
        }
    }

    for ai in a.iter() {
        print!("{} ", ai);
    }
    println!();
}
