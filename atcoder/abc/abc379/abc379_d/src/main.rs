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

    let mut v = Vec::new();
    let mut vg = 0;
    let mut tt = 0;

    let q: usize = tokens.next();
    for _ in 0..q {
        let x: usize = tokens.next();
        match x {
            1 => {
                v.push(tt);
            },
            2 => {
                let t: usize = tokens.next();
                tt += t;
            },
            3 => {
                let h: usize = tokens.next();
                if tt >= h {
                    let th = tt - h;
                    let vc = v.partition_point(|x| *x <= th);
                    if vc >= vg {
                        println!("{}", vc - vg);
                        vg = vc;
                    }
                    else {
                        println!("0");
                    }
                }
                else {
                    println!("0");
                }
            },
            _ => {panic!();}
        }

    }
    #[cfg(feature = "template")]
    {
        let a = tokens.next_string(); // String
        let b = tokens.next_bytes(); // Vec<u8>
        let n: usize = tokens.next();
        let v: Vec<u32> = tokens.collect(n);
        let i: Vec<(usize, u32)> = tokens.collect_index(n);
    
        debug!(a, b, n, v, i);
    
        let b0 = char::from_u32(b[0] as u32).unwrap(); // u8 -> char
        let bs = String::from_utf8(b).unwrap(); // Vec<u8> -> String
    
        debug!(b0, bs);

        let split = tokens.release();
        let r = split.count();
    
        println!("{r}");
    }
}
