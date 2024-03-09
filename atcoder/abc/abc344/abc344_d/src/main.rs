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

    let t = tokens.next_string(); // String
    let n: usize = tokens.next();
    let mut v = vec![Vec::new(); n];
    for i in 0..n {
        let a: usize = tokens.next();

        for _ in 0..a {
            let s = tokens.next_string(); // String

            v[i].push(s);
        }
    }
    debug!(v);

    let mut m = vec![-1; t.len() + 1];
    m[0] = 0;

    for i in 0..n {
        let mut mm = m.clone();

        for j in 0..t.len() {
            if m[j] >= 0 {
                let st = t.split_at(j).1;
                //debug!(st);

                for s in v[i].iter() {
                    //debug!(s);
                    if st.starts_with(s) {
                        //debug!(m[j + s.len()], m[j]);
                        if mm[j + s.len()] < 0 || mm[j + s.len()] > m[j] + 1 {
                            mm[j + s.len()] = m[j] + 1;
                        }
                    }
                }
            }
        }
        
        m = mm;
    }

    debug!(m);
    println!("{}", m[t.len()]);
}
