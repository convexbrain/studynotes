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

    let h: usize = tokens.next();
    let w: usize = tokens.next();
    let mut st = Vec::new();
    for i in 0..h {
        let s = tokens.next_string(); // String
        let v: Vec<char> = s.chars().collect();
        st.push(v);
    }

    let mut que = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if st[i][j] == 'E' {
                st[i][j] = '.';
                que.push_back((i, j, 'E'));
            }
        }
    }

    while let Some(q) = que.pop_front() {
        if st[q.0][q.1] == '.' {
            st[q.0][q.1] = q.2;
            if q.0 > 0 {
                que.push_back((q.0 - 1, q.1, 'v'));
            }
            if q.0 < h - 1 {
                que.push_back((q.0 + 1, q.1, '^'));
            }
            if q.1 > 0 {
                que.push_back((q.0, q.1 - 1, '>'));
            }
            if q.1 < w - 1 {
                que.push_back((q.0, q.1 + 1, '<'));
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", st[i][j]);
        }
        println!();
    }
}
