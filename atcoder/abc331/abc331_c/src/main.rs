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

    let n: usize = token.next().unwrap().parse().unwrap();

    let mut a: Vec<(usize, i64)> = (0..n).map(|i| (
        i,
        token.next().unwrap().parse().unwrap()
    )).collect();
    debug!(a);

    a.sort_by_key(|x| -x.1);
    debug!(a);

    let mut sum = 0;
    let mut fill = 0;
    let mut prev = a[0].1;
    for e in a.iter_mut() {
        let ns = sum + e.1;
        let np = e.1;
        if prev > e.1 {
            fill = sum;
        }

        e.1 = fill;

        prev = np;
        sum = ns;
    }

    a.sort_by_key(|x| x.0);
    print!("{}", a[0].1);
    for i in 1..n {
        print!(" {}", a[i].1);
    }
    println!();
}
