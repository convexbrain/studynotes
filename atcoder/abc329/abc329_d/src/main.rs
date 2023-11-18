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
    let m: usize = token.next().unwrap().parse().unwrap();

    let mut v = vec![0; n];
    let mut q = BinaryHeap::new();

    for _ in 0..m {
        let a: usize = token.next().unwrap().parse().unwrap();
        debug!(a);
        let a = a - 1;

        v[a] += 1;
        q.push((v[a], Reverse(a)));

        let (vm, am) = q.peek().unwrap();
        println!("{}", am.0 + 1);
    }
}
