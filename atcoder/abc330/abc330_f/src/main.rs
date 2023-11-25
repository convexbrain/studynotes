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
    let k: usize = token.next().unwrap().parse().unwrap();

    let mut xs: Vec<usize> = Vec::new();
    let mut ys: Vec<usize> = Vec::new();

    for _ in 0..n {
        xs.push(token.next().unwrap().parse().unwrap());
        ys.push(token.next().unwrap().parse().unwrap());
    }

    let mut cxs = xs.clone();
    let mut cys = ys.clone();
    cxs.sort();
    cys.sort();

    let mx2 = if n % 2 == 0 {cxs[n / 2] + cxs[n / 2 - 1]} else {cxs[n / 2] * 2};
    let my2 = if n % 2 == 0 {cys[n / 2] + cys[n / 2 - 1]} else {cys[n / 2] * 2};

    let mut pq = BinaryHeap::new();

    for i in 0..n {
        let mdx2 = (xs[i] * 2).abs_diff(mx2) + (ys[i] * 2).abs_diff(my2);
        pq.push(mdx2);
    }

    for _ in 0..k {
        let d = pq.pop().unwrap();
        pq.push(d.max(1) - 1);
    }

    let ans = pq.pop().unwrap() / 2;
    println!("{ans}");
}
