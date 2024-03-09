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
    let a: Vec<u32> = tokens.collect(n);

    let mut m = BTreeMap::new();

    for i in 0..n {
        let prev = if i > 0 {Some(a[i - 1])} else {None};
        let next = if i < n - 1 {Some(a[i + 1])} else {None};
        m.insert(a[i], (prev, next));
    }

    let mut head = a[0];

    let q: usize = tokens.next();
    for _ in 0..q {
        let t: usize = tokens.next();
        match t {
            1 => {
                let x: u32 = tokens.next();
                let y: u32 = tokens.next();

                let mx = m[&x];
                m.insert(x, (mx.0, Some(y)));
                m.insert(y, (Some(x), mx.1));

                if let Some(x1) = mx.1 {
                    let mx1 = m[&x1];
                    m.insert(x1, (Some(y), mx1.1));
                }
            },
            2 => {
                let x: u32 = tokens.next();

                let mx = m[&x];

                if head == x {
                    head = mx.1.unwrap();
                }

                if let Some(x0) = mx.0 {
                    let mx0 = m[&x0];
                    m.insert(x0, (mx0.0, mx.1));
                }

                if let Some(x1) = mx.1 {
                    let mx1 = m[&x1];
                    m.insert(x1, (mx.0, mx1.1));
                }

                m.remove(&x);
            },
            _ => { panic!(); },
        }
    }
    //debug!(m);

    let mut aa = Some(head);
    while let Some(aaa) = aa {
        print!("{aaa} ");
        aa = m[&aaa].1;
    }
    println!();
}
