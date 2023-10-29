use std::prelude::rust_2021::*;
use std::io::prelude::*;
use std::{collections::*, ops::*};
use std::{rc::*, cell::*, ops::Bound::*};

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

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    let mut token = buf.split_whitespace();

    let k: usize = token.next().unwrap().parse().unwrap();

    debug!(k);

    if k % 9 != 0 {
        println!("0");
        return;
    }

    let p = 1000000007;

    let mut cnt = vec![0; k + 1];

    for i in 1..=k {
        for d in 1..=9 {
            let r = i as isize - d as isize;
            if r == 0 {
                cnt[i] = (cnt[i] + 1) % p;
            }
            else if r > 0 {
                cnt[i] = (cnt[i] + cnt[r as usize]) % p;
            }
        }
    }

    println!("{}", cnt[k]);
}
