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

fn sub(
    tree: &[(bool, HashSet<usize>)],
    table: &mut[[u64; 3]],
    node: usize,
    prev: Option<usize>,
) {
    let p = 1_000_000_000 + 7;

    for &ch in tree[node].1.iter() {
        if prev.is_none() || prev.unwrap() != ch {
            sub(tree, table, ch, Some(node));
        }
    }

    if tree[node].0 {
        let mut table0 = 1_u64;
        let mut table2 = 1_u64;
        for &ch in tree[node].1.iter() {
            if prev.is_none() || prev.unwrap() != ch {
                let v0 = (table[ch][0] + table[ch][2]) % p;
                table0 = (table0 * v0) % p;

                let v2 = (2 * table[ch][2]) % p;
                let v2 = (v2 + table[ch][1]) % p;
                let v2 = (v2 + table[ch][0]) % p;
                table2 = (table2 * v2) % p;
            }
        }
        table2 = if table2 > table0 {(table2 - table0) % p} else {(table2 + p - table0) % p};

        table[node][0] = table0;
        table[node][1] = 0;
        table[node][2] = table2;
    }
    else {
        let mut table1 = 1_u64;
        let mut table2 = 1_u64;
        for &ch in tree[node].1.iter() {
            if prev.is_none() || prev.unwrap() != ch {
                let v1 = (table[ch][1] + table[ch][2]) % p;
                table1 = (table1 * v1) % p;

                let v2 = (2 * table[ch][2]) % p;
                let v2 = (v2 + table[ch][1]) % p;
                let v2 = (v2 + table[ch][0]) % p;
                table2 = (table2 * v2) % p;
            }
        }
        table2 = if table2 > table1 {(table2 - table1) % p} else {(table2 + p - table1) % p};

        table[node][0] = 0;
        table[node][1] = table1;
        table[node][2] = table2;
    }
}

fn main() {
    let mut tokens_buf = String::new();
    let mut tokens = Tokens::new(&mut tokens_buf);

    let n: usize = tokens.next();
    debug!(n);

    let mut tree = vec![(false, HashSet::new()); n];

    for i in 0..n {
        let c: char = tokens.next();
        tree[i].0 = c == 'a';
    }

    for _ in 0..(n - 1) {
        let a: usize = tokens.next();
        let b: usize = tokens.next();
        let a = a - 1;
        let b = b - 1;
        tree[a].1.insert(b);
        tree[b].1.insert(a);
    }
    debug!(tree);

    let mut table = vec![[0_u64; 3]; n];
    sub(&tree, &mut table, 0, None);

    println!("{}", table[0][2]);
}
