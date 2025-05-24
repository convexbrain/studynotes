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

    let t: usize = tokens.next();
    for _ in 0..t {
        let n: usize = tokens.next();
        let a: Vec<u64> = tokens.collect(2 * n);
        debug!(a);

        let mut sum_max = 0;
        let mut que = VecDeque::new();
        que.push_back((a[0], 1, 1));
        while let Some((sum, open, idx)) = que.pop_front() {
            let rest = 2 * n - idx;
            assert!(open <= rest);
            if open == rest {
                sum_max = sum_max.max(sum);
            }
            else {
                que.push_back((sum + a[idx], open + 1, idx + 1));
                if open > 0 {
                    que.push_back((sum, open - 1, idx + 1));
                }
            }
        }
        println!("{sum_max}");
    }
}
