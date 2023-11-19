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
    debug!(n, k);

    let mut a = vec![0_usize; n + 1];

    for i in 2..=n {
        if a[i] > 0 {
            continue;
        }
        else {
            let mut ii = i;
            while ii <= n {
                a[ii] += 1;
                ii += i;
            }
        }
    }
    debug!(a);

    let ans = a.iter().fold(0, |acc: usize, &x| if x >= k {acc + 1} else {acc});
    println!("{ans}");
}
