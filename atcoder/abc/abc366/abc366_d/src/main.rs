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
    let mut a = vec![vec![vec![0; n]; n]; n];
    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                let v: u64 = tokens.next();
                a[x][y][z] = v;
            }
        }
    }

    let mut s = vec![vec![vec![0; n]; n]; n];
    for x in 0..n {
        for y in 0..n {
            let mut sz = 0;
            for z in 0..n {
                sz += a[x][y][z];
                s[x][y][z] = sz;
            }
            if y > 0 {
                for z in 0..n {
                    s[x][y][z] += s[x][y - 1][z];
                }
            }
        }
        if x > 0 {
            for y in 0..n {
                for z in 0..n {
                    s[x][y][z] += s[x - 1][y][z];
                }
            }
    
        }
    }
    debug!(s);

    let q: usize = tokens.next();
    for _ in 0..q {
        let lx: usize = tokens.next();
        let rx: usize = tokens.next();
        let ly: usize = tokens.next();
        let ry: usize = tokens.next();
        let lz: usize = tokens.next();
        let rz: usize = tokens.next();
        let lx = lx - 1;
        let rx = rx - 1;
        let ly = ly - 1;
        let ry = ry - 1;
        let lz = lz - 1;
        let rz = rz - 1;
        //debug!(lx, rx);
        //debug!(ly, ry);
        //debug!(lz, rz);

        let sss = s[rx][ry][rz];

        let sss = sss + if ly > 0 && lz > 0 {s[rx][ly - 1][lz - 1]} else {0};
        let sss = sss + if lx > 0 && lz > 0 {s[lx - 1][ry][lz - 1]} else {0};
        let sss = sss + if lx > 0 && ly > 0 {s[lx - 1][ly - 1][rz]} else {0};

        let sss = sss - if lx > 0 {s[lx - 1][ry][rz]} else {0};
        let sss = sss - if ly > 0 {s[rx][ly - 1][rz]} else {0};
        let sss = sss - if lz > 0 {s[rx][ry][lz - 1]} else {0};

        let sss = sss - if lx > 0 && ly > 0 && lz > 0 {s[lx][ly][lz]} else {0};

        println!("{sss}");
    }
}
