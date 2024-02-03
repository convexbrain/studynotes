use std::prelude::rust_2021::*;
use std::io::prelude::*;

#[allow(unused_imports)]
use std::{
    collections::*, ops::{*, Bound::*}, cmp::*,
    str, rc::*, cell::*,
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

//#############################################################################

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let m: usize = token.next().unwrap().parse().unwrap();
    let d: usize = token.next().unwrap().parse().unwrap();
    let mut yy: usize = token.next().unwrap().parse().unwrap();
    let mut mm: usize = token.next().unwrap().parse().unwrap();
    let mut dd: usize = token.next().unwrap().parse().unwrap();

    dd = dd + 1;
    if dd > d {
        dd = 1;
        mm += 1;
    }
    if mm > m {
        mm = 1;
        yy += 1;
    }
    
    println!("{yy} {mm} {dd}");
}
