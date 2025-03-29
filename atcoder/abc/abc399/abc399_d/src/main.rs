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

    let t: usize = tokens.next();
    for _ in 0..t {
        let n: usize = tokens.next();
        let a: Vec<usize> = tokens.collect(n * 2);

        let mut v = vec![Vec::new(); n];
        for (pos, num) in a.iter().enumerate() {
            let aa = num - 1;
            v[aa].push(pos);
        }

        let mut pc = BTreeSet::new();
        for i in 1..(n * 2) {
            let p = if a[i - 1] < a[i] {(a[i - 1], a[i])} else {(a[i], a[i - 1])};
            pc.insert((p.0 - 1, p.1 - 1));
        }

        let mut ans = 0;
        for (n0, n1) in pc.iter() {
            let n0 = *n0;
            let n1 = *n1;
            if v[n0][0].abs_diff(v[n0][1]) == 1 {
                //
            }
            else if v[n1][0].abs_diff(v[n1][1]) == 1 {
                //
            }
            else if v[n0][0].abs_diff(v[n1][0]) == 1 && v[n0][1].abs_diff(v[n1][1]) == 1 {
                ans += 1;
            }
            else if v[n0][0].abs_diff(v[n1][1]) == 1 && v[n0][1].abs_diff(v[n1][0]) == 1 {
                ans += 1;
            }
        }
        println!("{ans}");
    }
}
