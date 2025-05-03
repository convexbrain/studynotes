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
    fn new(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_to_string(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
    }
    fn new_line(placeholder: &'a mut String) -> Self {
        placeholder.clear();
        std::io::stdin().read_line(placeholder).unwrap();
        Tokens(placeholder.split_whitespace())
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
}

//#############################################################################

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let mut ss = vec![vec![0_u8; n]; n];
    let mut tt = vec![vec![0_u8; n]; n];
    for i in 0..n {
        let s = tokens.next_string(); // String
        for (j, c) in s.char_indices() {
            ss[i][j] = if c == '#' {0} else {1};
        }
    }
    for i in 0..n {
        let t = tokens.next_string(); // String
        for (j, c) in t.char_indices() {
            tt[i][j] = if c == '#' {0} else {1};
        }
    }

    let mut ans = Vec::new();
    {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if ss[i][j] != tt[i][j] {
                    cnt += 1;
                }
            }
        }
        ans.push(cnt);
    }
    {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if ss[n - 1 - j][i] != tt[i][j] {
                    cnt += 1;
                }
            }
        }
        ans.push(cnt + 1);
    }
    {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if ss[n - 1 - i][n - 1 - j] != tt[i][j] {
                    cnt += 1;
                }
            }
        }
        ans.push(cnt + 2);
    }
    {
        let mut cnt = 0;
        for i in 0..n {
            for j in 0..n {
                if ss[j][n - 1 - i] != tt[i][j] {
                    cnt += 1;
                }
            }
        }
        ans.push(cnt + 3);
    }
    ans.sort();
    println!("{}", ans[0]);
}
