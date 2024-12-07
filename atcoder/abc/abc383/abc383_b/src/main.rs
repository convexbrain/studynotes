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

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let d: usize = tokens.next();

    let mut f = BTreeSet::new();

    for i in 0..h {
        let s = tokens.next_string(); // String
        for (j, c) in s.char_indices() {
            if c == '.' {
                f.insert((i, j));
            }
        }
    }

    let mut ans = 0_usize;
    for i1 in 0..h {
        for j1 in 0..w {
            if !f.contains(&(i1, j1)) {
                continue;
            }

            for i2 in 0..h {
                for j2 in 0..w {
                    if !f.contains(&(i2, j2)) || (i1, j1) == (i2, j2) {
                        continue;
                    }
        
                    let mut cnt = 0;
                    for i in 0..h {
                        for j in 0..w {
                            let d1 = i.abs_diff(i1) + j.abs_diff(j1);
                            let d2 = i.abs_diff(i2) + j.abs_diff(j2);
                            if f.contains(&(i, j)) && (d1 <= d || d2 <= d) {
                                cnt += 1;
                            }
                        }
                    }

                    ans = ans.max(cnt);
                }
            }
        }
    }

    println!("{ans}");
}
