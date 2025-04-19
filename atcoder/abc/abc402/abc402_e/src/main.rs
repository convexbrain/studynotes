use std::{hash::Hash, prelude::rust_2021::*};
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

fn sub(scp: &[(u32, u16, u8)], memo: &mut BTreeMap<(u8, u16), f64>, vis: &mut u8, rx: u16) -> f64 {
    if let Some(sc) = memo.get(&(*vis, rx)) {
        *sc
    }
    else {
        let n = scp.len();

        let mut sc_max = 0_f64;
        for i in 0..n {
            if *vis & (1 << i) == 0 {
                if rx >= scp[i].1 {
                    let rx = rx - scp[i].1;
        
                    let sc0 = sub(scp, memo, vis, rx) * (100 - scp[i].2) as f64 / 100.;
        
                    *vis |= 1 << i;
                    let sc1 = (scp[i].0 as f64 + sub(scp, memo, vis, rx)) * scp[i].2 as f64 / 100.;
                    *vis ^= 1 << i;
        
                    sc_max = sc_max.max(sc0 + sc1)
                }
            }
        }

        memo.insert((*vis, rx), sc_max);
        sc_max
    }
}

fn main() {
    let mut placeholder = String::new();
    let mut tokens = Tokens::new(&mut placeholder);

    let n: usize = tokens.next();
    let x: u16 = tokens.next();
    let scp: Vec<(u32, u16, u8)> = (0..n).map(|i|
        (tokens.next(), tokens.next(), tokens.next())
    ).collect();

    let mut memo = BTreeMap::new();
    let mut vis = 0;
    let ans = sub(&scp, &mut memo, &mut vis, x);
    println!("{ans}");
}
