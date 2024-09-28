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

fn sub(map: &BTreeMap<isize, usize>, m: usize, ai: isize, c: isize, rest: isize) -> bool {
    if let Some((key1, val1)) = map.range((Excluded(ai + c), Unbounded)).next() {
        if *val1 < m {
            if let Some((key2, val2)) = map.range((Unbounded, Included(ai + c))).last() {
                if c == 0 && *val2 == *val1 + 1 {
                    if let Some((key3, val3)) = map.range((Unbounded, Excluded(ai + c))).last() {
                        if ai + c < *key3 + rest {
                            false
                        }
                        else {
                            debug!(c);
                            true
                        }
                    }
                    else {
                        debug!(c);
                        true
                    }
                }
                else {
                    if ai + c < *key2 + rest {
                        false
                    }
                    else {
                        debug!(c);
                        true
                    }
                }
            }
            else {
                assert_ne!(c, 0);
                debug!(c);
                true
            }
        }
        else {
            false
        }
    }
    else {
        debug!(c);
        true
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    let m: usize = tokens.next();
    let k: isize = tokens.next();
    let a: Vec<isize> = tokens.collect(n);

    let ka: isize = k - a.iter().sum::<isize>();

    let mut map = BTreeMap::<isize, usize>::new();
    {
        let mut s = a.clone();
        s.sort();
        let mut cnt = 0;
        for &si in s.iter().rev() {
            cnt += 1;
            map.entry(si).and_modify(|e| {
                *e = cnt;
            }).or_insert(cnt);
        }
    }
    debug!(map);

    for &ai in a.iter() {
        debug!(ai);
        if sub(&map, m, ai, ka, 0) {
            debug!(ka);
            let mut l = -1;
            let mut r = ka;

            while l + 1 < r {
                let c = (l + r) / 2;
                if sub(&map, m, ai, c, ka - c) {
                    r = c;
                    debug!(l, r);
                }
                else {
                    l = c;
                    debug!(l, r);
                }
            }
    
            print!("{r} ");
        }
        else {
            print!("-1 ");
        }
    }
    println!();
}
