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
    let q: usize = token.next().unwrap().parse().unwrap();

    let mut bx = vec![BTreeSet::new(); n];

    for i in 0..n {
        let c: usize = token.next().unwrap().parse().unwrap();
        debug!(c);

        bx[i].insert(c);
    }

    let mut give = BTreeSet::new();

    for _ in 0..q {
        let a: usize = token.next().unwrap().parse().unwrap();
        let b: usize = token.next().unwrap().parse().unwrap();
        debug!(a, b);
        let a = a - 1;
        let b = b - 1;

        give.append(&mut bx[a]);
        bx[b].append(&mut give);

        let ans = bx[b].len();

        println!("{ans}");
    }
}
